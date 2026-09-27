//! QUIC transport engine for unreliable datagram transport

use crate::{
    config::{TransportConfig, make_client_config, make_server_config},
    error::{Result, TransportError},
};
use bytes::Bytes;
use oxide_core::NodeId;
use oxide_protocol::WirePacket;
use quinn::{Connection, Endpoint, RecvStream, SendStream};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::{
    sync::{RwLock, broadcast, mpsc, oneshot},
    task::JoinHandle,
};
use tracing::{debug, info, warn};

/// Transport event types
#[derive(Debug, Clone)]
pub enum TransportEvent {
    /// New peer connected
    Connected {
        node_id: NodeId,
        endpoint: SocketAddr,
    },
    /// Peer disconnected
    Disconnected { node_id: NodeId, reason: String },
    /// Datagram received
    DatagramReceived { from: NodeId, packet: WirePacket },
    /// Stream opened
    StreamOpened { node_id: NodeId, stream_id: u64 },
    /// Stream data received
    StreamData {
        node_id: NodeId,
        stream_id: u64,
        data: Bytes,
        fin: bool,
    },
    /// Stream closed
    StreamClosed { node_id: NodeId, stream_id: u64 },
    /// Connection migrated (IP change)
    Migrated {
        node_id: NodeId,
        new_endpoint: SocketAddr,
    },
}

/// Transport handle for sending data
#[derive(Clone)]
pub struct TransportHandle {
    event_tx: broadcast::Sender<TransportEvent>,
    command_tx: mpsc::UnboundedSender<TransportCommand>,
}

/// Internal transport commands
enum TransportCommand {
    Connect {
        node_id: NodeId,
        endpoint: SocketAddr,
        response: oneshot::Sender<Result<()>>,
    },
    Disconnect {
        node_id: NodeId,
        response: oneshot::Sender<()>,
    },
    SendDatagram {
        node_id: NodeId,
        packet: WirePacket,
        response: oneshot::Sender<Result<()>>,
    },
    OpenStream {
        node_id: NodeId,
        response: oneshot::Sender<Result<SendStream>>,
    },
    GetStats {
        response: oneshot::Sender<TransportStats>,
    },
    Shutdown,
}

/// Transport statistics
#[derive(Debug, Clone, Default)]
pub struct TransportStats {
    pub active_connections: usize,
    pub total_connections: u64,
    pub datagrams_sent: u64,
    pub datagrams_received: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub streams_opened: u64,
    pub streams_closed: u64,
    pub connection_errors: u64,
}

/// Connection state
struct ConnectionState {
    connection: Connection,
    #[allow(dead_code)]
    node_id: NodeId,
    remote_endpoint: SocketAddr,
    last_activity: Instant,
    stats: ConnectionStats,
}

/// Per-connection statistics
#[derive(Default)]
struct ConnectionStats {
    datagrams_sent: u64,
    datagrams_received: u64,
    bytes_sent: u64,
    bytes_received: u64,
}

/// Main transport engine
pub struct TransportEngine {
    config: TransportConfig,
    endpoint: Option<Endpoint>,
    connections: Arc<RwLock<HashMap<NodeId, ConnectionState>>>,
    endpoint_by_addr: Arc<RwLock<HashMap<SocketAddr, NodeId>>>,
    event_tx: broadcast::Sender<TransportEvent>,
    command_rx: Option<mpsc::UnboundedReceiver<TransportCommand>>,
    stats: Arc<RwLock<TransportStats>>,
    _worker_handles: Vec<JoinHandle<()>>,
}

impl TransportEngine {
    /// Create a new transport engine
    pub fn new(config: TransportConfig) -> Result<(Self, TransportHandle)> {
        let (event_tx, _) = broadcast::channel(4096);
        let (command_tx, command_rx) = mpsc::unbounded_channel();

        let engine = Self {
            config,
            endpoint: None,
            connections: Arc::new(RwLock::new(HashMap::new())),
            endpoint_by_addr: Arc::new(RwLock::new(HashMap::new())),
            event_tx: event_tx.clone(),
            command_rx: Some(command_rx),
            stats: Arc::new(RwLock::new(TransportStats::default())),
            _worker_handles: Vec::new(),
        };

        let handle = TransportHandle {
            event_tx,
            command_tx,
        };

        Ok((engine, handle))
    }

    /// Start the transport engine
    pub async fn start(&mut self) -> Result<()> {
        let client_config = make_client_config(&self.config)?;
        let mut endpoint = if self.config.server_cert.is_some() {
            let server_config = make_server_config(&self.config)?;
            Endpoint::server(server_config, self.config.bind_addrs[0])?
        } else {
            Endpoint::client(self.config.bind_addrs[0])?
        };
        endpoint.set_default_client_config(client_config);

        self.endpoint = Some(endpoint.clone());

        let command_rx = self
            .command_rx
            .take()
            .ok_or_else(|| TransportError::Internal("TransportEngine already started".into()))?;

        // Start command processor
        let connections = self.connections.clone();
        let endpoint_by_addr = self.endpoint_by_addr.clone();
        let stats = self.stats.clone();
        let event_tx = self.event_tx.clone();
        let endpoint_clone = endpoint.clone();
        let config = self.config.clone();

        let worker = tokio::spawn(async move {
            Self::command_processor(
                connections,
                endpoint_by_addr,
                stats,
                event_tx,
                endpoint_clone,
                config,
                command_rx,
            )
            .await;
        });

        self._worker_handles.push(worker);

        // Start connection monitor
        let connections = self.connections.clone();
        let endpoint_by_addr = self.endpoint_by_addr.clone();
        let event_tx = self.event_tx.clone();
        let idle_timeout = self.config.idle_timeout;

        let monitor = tokio::spawn(async move {
            Self::connection_monitor(connections, endpoint_by_addr, event_tx, idle_timeout).await;
        });

        self._worker_handles.push(monitor);

        // Accept incoming connections if server
        if self.config.server_cert.is_some() {
            let connections = self.connections.clone();
            let endpoint_by_addr = self.endpoint_by_addr.clone();
            let stats = self.stats.clone();
            let event_tx = self.event_tx.clone();
            let endpoint_in = endpoint.clone();

            let accept_worker = tokio::spawn(async move {
                while let Some(incoming) = endpoint_in.accept().await {
                    let connections = connections.clone();
                    let endpoint_by_addr = endpoint_by_addr.clone();
                    let stats = stats.clone();
                    let event_tx = event_tx.clone();

                    tokio::spawn(async move {
                        if let Ok(connection) = incoming.await {
                            let remote = connection.remote_address();
                            let node_id = NodeId::new();
                            let state = ConnectionState {
                                connection,
                                node_id,
                                remote_endpoint: remote,
                                last_activity: Instant::now(),
                                stats: ConnectionStats::default(),
                            };
                            connections.write().await.insert(node_id, state);
                            endpoint_by_addr.write().await.insert(remote, node_id);
                            event_tx
                                .send(TransportEvent::Connected {
                                    node_id,
                                    endpoint: remote,
                                })
                                .ok();

                            let c1 = connections.clone();
                            let s1 = stats.clone();
                            let e1 = event_tx.clone();
                            tokio::spawn(async move {
                                Self::datagram_receiver(c1, s1, e1, node_id).await;
                            });

                            let c2 = connections.clone();
                            let s2 = stats.clone();
                            let e2 = event_tx.clone();
                            tokio::spawn(async move {
                                Self::stream_receiver(c2, s2, e2, node_id).await;
                            });
                        }
                    });
                }
            });
            self._worker_handles.push(accept_worker);
        }

        info!("Transport engine started");
        Ok(())
    }

    /// Subscribe to transport events
    pub fn subscribe(&self) -> broadcast::Receiver<TransportEvent> {
        self.event_tx.subscribe()
    }

    /// Command processor loop
    async fn command_processor(
        connections: Arc<RwLock<HashMap<NodeId, ConnectionState>>>,
        endpoint_by_addr: Arc<RwLock<HashMap<SocketAddr, NodeId>>>,
        stats: Arc<RwLock<TransportStats>>,
        event_tx: broadcast::Sender<TransportEvent>,
        endpoint: Endpoint,
        _config: TransportConfig,
        mut command_rx: mpsc::UnboundedReceiver<TransportCommand>,
    ) {
        while let Some(cmd) = command_rx.recv().await {
            match cmd {
                TransportCommand::Connect {
                    node_id,
                    endpoint: addr,
                    response,
                } => {
                    let result = Self::connect_inner(
                        &connections,
                        &endpoint_by_addr,
                        &stats,
                        &event_tx,
                        &endpoint,
                        node_id,
                        addr,
                    )
                    .await;
                    let _ = response.send(result);
                }
                TransportCommand::Disconnect { node_id, response } => {
                    Self::disconnect_inner(&connections, &endpoint_by_addr, node_id).await;
                    let _ = response.send(());
                }
                TransportCommand::SendDatagram {
                    node_id,
                    packet,
                    response,
                } => {
                    let result =
                        Self::send_datagram_inner(&connections, &stats, node_id, packet).await;
                    let _ = response.send(result);
                }
                TransportCommand::OpenStream { node_id, response } => {
                    let result = Self::open_stream_inner(&connections, node_id).await;
                    let _ = response.send(result);
                }
                TransportCommand::GetStats { response } => {
                    let s = stats.read().await.clone();
                    let _ = response.send(s);
                }
                TransportCommand::Shutdown => {
                    info!("Transport engine shutting down");
                    break;
                }
            }
        }
    }

    /// Connect to a peer
    async fn connect_inner(
        connections: &Arc<RwLock<HashMap<NodeId, ConnectionState>>>,
        endpoint_by_addr: &Arc<RwLock<HashMap<SocketAddr, NodeId>>>,
        stats: &Arc<RwLock<TransportStats>>,
        event_tx: &broadcast::Sender<TransportEvent>,
        endpoint: &Endpoint,
        node_id: NodeId,
        addr: SocketAddr,
    ) -> Result<()> {
        // Check if already connected
        {
            let conns = connections.read().await;
            if conns.contains_key(&node_id) {
                return Ok(());
            }
        }

        debug!("Connecting to {} at {}", node_id, addr);

        let connect_fut = endpoint.connect(addr, "oxide-mesh")?;
        let connection = connect_fut
            .await
            .map_err(|e| TransportError::ConnectionFailed(e.to_string()))?;

        let state = ConnectionState {
            connection,
            node_id,
            remote_endpoint: addr,
            last_activity: Instant::now(),
            stats: ConnectionStats::default(),
        };

        {
            let mut conns = connections.write().await;
            conns.insert(node_id, state);
        }

        {
            let mut addrs = endpoint_by_addr.write().await;
            addrs.insert(addr, node_id);
        }

        {
            let mut s = stats.write().await;
            s.active_connections += 1;
            s.total_connections += 1;
        }

        event_tx
            .send(TransportEvent::Connected {
                node_id,
                endpoint: addr,
            })
            .ok();

        // Spawn datagram receiver
        let c1 = connections.clone();
        let s1 = stats.clone();
        let e1 = event_tx.clone();

        tokio::spawn(async move {
            Self::datagram_receiver(c1, s1, e1, node_id).await;
        });

        // Spawn stream receiver
        let c2 = connections.clone();
        let s2 = stats.clone();
        let e2 = event_tx.clone();

        tokio::spawn(async move {
            Self::stream_receiver(c2, s2, e2, node_id).await;
        });

        Ok(())
    }

    /// Disconnect from a peer
    async fn disconnect_inner(
        connections: &Arc<RwLock<HashMap<NodeId, ConnectionState>>>,
        endpoint_by_addr: &Arc<RwLock<HashMap<SocketAddr, NodeId>>>,
        node_id: NodeId,
    ) {
        let mut conns = connections.write().await;
        if let Some(state) = conns.remove(&node_id) {
            state.connection.close(0u32.into(), b"disconnect");
            let mut addrs = endpoint_by_addr.write().await;
            addrs.remove(&state.remote_endpoint);
        }
    }

    /// Send a datagram
    async fn send_datagram_inner(
        connections: &Arc<RwLock<HashMap<NodeId, ConnectionState>>>,
        stats: &Arc<RwLock<TransportStats>>,
        node_id: NodeId,
        packet: WirePacket,
    ) -> Result<()> {
        let conns = connections.read().await;
        let state = conns.get(&node_id).ok_or(TransportError::NotConnected)?;

        let bytes = packet.to_bytes();
        state.connection.send_datagram(bytes.into())?;

        {
            let mut s = stats.write().await;
            s.datagrams_sent += 1;
            s.bytes_sent += packet.total_len() as u64;
        }

        // Update connection stats
        drop(conns);
        let mut conns = connections.write().await;
        if let Some(state) = conns.get_mut(&node_id) {
            state.stats.datagrams_sent += 1;
            state.stats.bytes_sent += packet.total_len() as u64;
            state.last_activity = Instant::now();
        }

        Ok(())
    }

    /// Open a bidirectional stream
    async fn open_stream_inner(
        connections: &Arc<RwLock<HashMap<NodeId, ConnectionState>>>,
        node_id: NodeId,
    ) -> Result<SendStream> {
        let conns = connections.read().await;
        let state = conns.get(&node_id).ok_or(TransportError::NotConnected)?;

        let (send, _recv) = state
            .connection
            .open_bi()
            .await
            .map_err(|e| TransportError::StreamError(e.to_string()))?;

        Ok(send)
    }

    /// Datagram receiver loop
    async fn datagram_receiver(
        connections: Arc<RwLock<HashMap<NodeId, ConnectionState>>>,
        stats: Arc<RwLock<TransportStats>>,
        event_tx: broadcast::Sender<TransportEvent>,
        node_id: NodeId,
    ) {
        loop {
            let conn = {
                let conns = connections.read().await;
                conns.get(&node_id).map(|s| s.connection.clone())
            };

            let Some(connection) = conn else {
                break;
            };

            match connection.read_datagram().await {
                Ok(bytes) => {
                    if let Ok(packet) = WirePacket::from_bytes(&bytes) {
                        {
                            let mut s = stats.write().await;
                            s.datagrams_received += 1;
                            s.bytes_received += bytes.len() as u64;
                        }

                        let mut conns = connections.write().await;
                        if let Some(state) = conns.get_mut(&node_id) {
                            state.stats.datagrams_received += 1;
                            state.stats.bytes_received += bytes.len() as u64;
                            state.last_activity = Instant::now();
                        }

                        event_tx
                            .send(TransportEvent::DatagramReceived {
                                from: node_id,
                                packet,
                            })
                            .ok();
                    }
                }
                Err(quinn::ConnectionError::ApplicationClosed { .. }) => {
                    break;
                }
                Err(e) => {
                    warn!("Datagram read error for {}: {}", node_id, e);
                    break;
                }
            }
        }

        // Clean up
        let mut conns = connections.write().await;
        conns.remove(&node_id);
    }

    /// Stream receiver loop
    async fn stream_receiver(
        connections: Arc<RwLock<HashMap<NodeId, ConnectionState>>>,
        stats: Arc<RwLock<TransportStats>>,
        event_tx: broadcast::Sender<TransportEvent>,
        node_id: NodeId,
    ) {
        let conn = {
            let conns = connections.read().await;
            conns.get(&node_id).map(|s| s.connection.clone())
        };

        let Some(connection) = conn else {
            return;
        };

        loop {
            match connection.accept_bi().await {
                Ok((send, recv)) => {
                    let stream_id = send.id().index();
                    event_tx
                        .send(TransportEvent::StreamOpened { node_id, stream_id })
                        .ok();

                    let stats = stats.clone();
                    let event_tx = event_tx.clone();

                    tokio::spawn(async move {
                        Self::handle_stream(send, recv, stats, event_tx, node_id, stream_id).await;
                    });
                }
                Err(quinn::ConnectionError::ApplicationClosed { .. }) => {
                    break;
                }
                Err(e) => {
                    warn!("Stream accept error for {}: {}", node_id, e);
                    break;
                }
            }
        }
    }

    /// Handle a single stream
    async fn handle_stream(
        _send: SendStream,
        mut recv: RecvStream,
        stats: Arc<RwLock<TransportStats>>,
        event_tx: broadcast::Sender<TransportEvent>,
        node_id: NodeId,
        stream_id: u64,
    ) {
        let mut buffer = vec![0u8; 65536];

        loop {
            match recv.read(&mut buffer).await {
                Ok(Some(n)) => {
                    let data = Bytes::copy_from_slice(&buffer[..n]);

                    {
                        let mut s = stats.write().await;
                        s.bytes_received += n as u64;
                    }

                    event_tx
                        .send(TransportEvent::StreamData {
                            node_id,
                            stream_id,
                            data,
                            fin: false,
                        })
                        .ok();
                }
                Ok(None) => {
                    event_tx
                        .send(TransportEvent::StreamData {
                            node_id,
                            stream_id,
                            data: Bytes::new(),
                            fin: true,
                        })
                        .ok();
                    event_tx
                        .send(TransportEvent::StreamClosed { node_id, stream_id })
                        .ok();
                    break;
                }
                Err(e) => {
                    warn!("Stream read error: {}", e);
                    break;
                }
            }
        }
    }

    /// Monitor connection health and idle timeouts
    async fn connection_monitor(
        connections: Arc<RwLock<HashMap<NodeId, ConnectionState>>>,
        endpoint_by_addr: Arc<RwLock<HashMap<SocketAddr, NodeId>>>,
        event_tx: broadcast::Sender<TransportEvent>,
        idle_timeout: Duration,
    ) {
        let mut interval = tokio::time::interval(Duration::from_secs(5));

        loop {
            interval.tick().await;

            let now = Instant::now();
            let mut timed_out = Vec::new();

            {
                let conns = connections.read().await;
                for (node_id, state) in conns.iter() {
                    if now.duration_since(state.last_activity) > idle_timeout {
                        timed_out.push(*node_id);
                    }
                }
            }

            for node_id in timed_out {
                warn!("Connection timed out for {}", node_id);
                event_tx
                    .send(TransportEvent::Disconnected {
                        node_id,
                        reason: "Idle timeout".into(),
                    })
                    .ok();

                let mut conns = connections.write().await;
                if let Some(state) = conns.remove(&node_id) {
                    state.connection.close(0u32.into(), b"idle timeout");
                    let mut addrs = endpoint_by_addr.write().await;
                    addrs.remove(&state.remote_endpoint);
                }
            }
        }
    }
}

impl TransportHandle {
    /// Subscribe to transport events
    pub fn subscribe(&self) -> broadcast::Receiver<TransportEvent> {
        self.event_tx.subscribe()
    }

    /// Connect to a peer
    pub async fn connect(&self, node_id: NodeId, endpoint: SocketAddr) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.command_tx.send(TransportCommand::Connect {
            node_id,
            endpoint,
            response: tx,
        })?;
        rx.await
            .map_err(|_| TransportError::Internal("Command channel closed".into()))?
    }

    /// Disconnect from a peer
    pub async fn disconnect(&self, node_id: NodeId) {
        let (tx, rx) = oneshot::channel();
        let _ = self.command_tx.send(TransportCommand::Disconnect {
            node_id,
            response: tx,
        });
        let _ = rx.await;
    }

    /// Send a datagram
    pub async fn send_datagram(&self, node_id: NodeId, packet: WirePacket) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.command_tx.send(TransportCommand::SendDatagram {
            node_id,
            packet,
            response: tx,
        })?;
        rx.await
            .map_err(|_| TransportError::Internal("Command channel closed".into()))?
    }

    /// Open a bidirectional stream
    pub async fn open_stream(&self, node_id: NodeId) -> Result<SendStream> {
        let (tx, rx) = oneshot::channel();
        self.command_tx.send(TransportCommand::OpenStream {
            node_id,
            response: tx,
        })?;
        rx.await
            .map_err(|_| TransportError::Internal("Command channel closed".into()))?
    }

    /// Get transport statistics
    pub async fn stats(&self) -> TransportStats {
        let (tx, rx) = oneshot::channel();
        let _ = self
            .command_tx
            .send(TransportCommand::GetStats { response: tx });
        rx.await.unwrap_or_default()
    }

    /// Shutdown the transport
    pub fn shutdown(&self) {
        let _ = self.command_tx.send(TransportCommand::Shutdown);
    }
}
