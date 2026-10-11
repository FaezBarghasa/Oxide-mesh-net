use bytes::{BufMut, BytesMut};
use oxide_core::NodeId;
use oxide_protocol::WirePacket;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tracing::{info, warn};

/// Framing magic for framed WebSocket/TCP stream packets
pub const WS_RELAY_FRAME_MAGIC: u32 = 0x57534F58; // "WSOX"

/// CDN Edge Relay Profile configuration
#[derive(Debug, Clone)]
pub struct CdnEdgeConfig {
    /// Public CDN Domain (e.g., "relay.arvan.mydomain.ir" or "edge.parspack.mydomain.ir")
    pub edge_domain: String,
    /// Resolved SocketAddr of the CDN Anycast IP or edge proxy
    pub edge_addr: SocketAddr,
    /// Port (typically 443 for HTTPS / WSS)
    pub port: u16,
    /// WebSocket path on CDN edge (e.g. "/ws/v1/relay")
    pub ws_path: String,
    /// Optional fake Host header to pass through CDN SNI routing
    pub custom_host_header: Option<String>,
    /// Heartbeat keepalive interval to prevent CDN middlebox idle disconnect
    pub keepalive_interval: Duration,
}

impl Default for CdnEdgeConfig {
    fn default() -> Self {
        Self {
            edge_domain: "relay.oxide.mesh.ir".to_string(),
            edge_addr: "127.0.0.1:443".parse().unwrap(),
            port: 443,
            ws_path: "/ws/v1/relay".to_string(),
            custom_host_header: None,
            keepalive_interval: Duration::from_secs(15),
        }
    }
}

/// Statistics for CDN Edge tunnel
#[derive(Debug, Default)]
pub struct CdnTunnelStats {
    pub packets_sent: AtomicU64,
    pub packets_received: AtomicU64,
    pub bytes_sent: AtomicU64,
    pub bytes_received: AtomicU64,
    pub reconnects: AtomicU64,
}

/// Client tunnel for streaming packets through an Iranian CDN edge or TCP reverse-proxy
pub struct CdnEdgeTunnelClient {
    config: CdnEdgeConfig,
    local_node_id: NodeId,
    stats: Arc<CdnTunnelStats>,
    is_connected: Arc<AtomicBool>,
}

impl CdnEdgeTunnelClient {
    pub fn new(config: CdnEdgeConfig, local_node_id: NodeId) -> Self {
        Self {
            config,
            local_node_id,
            stats: Arc::new(CdnTunnelStats::default()),
            is_connected: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Craft standard HTTP/1.1 WebSocket Upgrade request formatted for CDN proxies (Arvan / ParsPack)
    pub fn build_ws_upgrade_request(&self) -> Vec<u8> {
        let host = self
            .config
            .custom_host_header
            .as_deref()
            .unwrap_or(&self.config.edge_domain);

        format!(
            "GET {} HTTP/1.1\r\n\
             Host: {}\r\n\
             Upgrade: websocket\r\n\
             Connection: Upgrade\r\n\
             Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n\
             Sec-WebSocket-Version: 13\r\n\
             X-Oxide-Node-Id: {}\r\n\
             \r\n",
            self.config.ws_path, host, self.local_node_id
        )
        .into_bytes()
    }

    /// Encode a WirePacket into length-prefixed CDN relay framing:
    /// [Magic (4B) | Target NodeId (16B) | Payload Len (4B) | Payload]
    pub fn encode_frame(target_node: NodeId, packet: &WirePacket) -> Vec<u8> {
        let raw_pkt = packet.to_bytes();
        let mut buf = BytesMut::with_capacity(24 + raw_pkt.len());
        buf.put_u32_le(WS_RELAY_FRAME_MAGIC);
        buf.put_slice(target_node.as_bytes());
        buf.put_u32_le(raw_pkt.len() as u32);
        buf.put_slice(&raw_pkt);
        buf.freeze().to_vec()
    }

    /// Decode a length-prefixed CDN relay frame
    pub fn decode_frame(buf: &[u8]) -> Option<(NodeId, WirePacket, usize)> {
        if buf.len() < 24 {
            return None;
        }
        let magic = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
        if magic != WS_RELAY_FRAME_MAGIC {
            return None;
        }
        let target_node = NodeId::from_bytes(buf[4..20].try_into().ok()?);
        let payload_len = u32::from_le_bytes([buf[20], buf[21], buf[22], buf[23]]) as usize;

        if buf.len() < 24 + payload_len {
            return None;
        }

        let packet = WirePacket::from_bytes(&buf[24..24 + payload_len]).ok()?;
        Some((target_node, packet, 24 + payload_len))
    }

    /// Connect and initiate bidirectional streaming over CDN Edge TCP/WSS tunnel
    pub async fn start_streaming(
        self: Arc<Self>,
        outbound_rx: mpsc::Receiver<(NodeId, WirePacket)>,
        inbound_tx: mpsc::Sender<(NodeId, WirePacket)>,
    ) {
        let config = self.config.clone();
        let stats = self.stats.clone();
        let connected = self.is_connected.clone();
        let this = self.clone();
        let outbound_rx = Arc::new(tokio::sync::Mutex::new(outbound_rx));

        tokio::spawn(async move {
            loop {
                info!(
                    "Connecting to CDN edge '{}:{}' via WSS transport...",
                    config.edge_domain, config.port
                );

                match TcpStream::connect(config.edge_addr).await {
                    Ok(mut stream) => {
                        let req = this.build_ws_upgrade_request();
                        if let Err(e) = stream.write_all(&req).await {
                            warn!("Failed sending WS upgrade to CDN edge: {}", e);
                            tokio::time::sleep(Duration::from_secs(2)).await;
                            continue;
                        }

                        let mut resp_buf = [0u8; 1024];
                        match stream.read(&mut resp_buf).await {
                            Ok(n) if n > 0 => {
                                let resp_str = String::from_utf8_lossy(&resp_buf[..n]);
                                if !resp_str.contains("101 Switching Protocols")
                                    && !resp_str.contains("200 OK")
                                {
                                    warn!("CDN Edge did not return 101/200: {}", resp_str);
                                    tokio::time::sleep(Duration::from_secs(2)).await;
                                    continue;
                                }
                            }
                            _ => {
                                warn!("Failed to read CDN edge handshake response");
                                tokio::time::sleep(Duration::from_secs(2)).await;
                                continue;
                            }
                        }

                        connected.store(true, Ordering::Relaxed);
                        stats.reconnects.fetch_add(1, Ordering::Relaxed);
                        info!("Connected to CDN Edge Relay successfully");

                        let (mut reader, mut writer) = stream.into_split();

                        // Task: Send outbound packets to CDN Edge
                        let stats_tx = stats.clone();
                        let rx_clone = outbound_rx.clone();
                        let send_task = tokio::spawn(async move {
                            loop {
                                let next_item = {
                                    let mut lock = rx_clone.lock().await;
                                    lock.recv().await
                                };
                                match next_item {
                                    Some((target, packet)) => {
                                        let frame = Self::encode_frame(target, &packet);
                                        if writer.write_all(&frame).await.is_err() {
                                            break;
                                        }
                                        stats_tx.packets_sent.fetch_add(1, Ordering::Relaxed);
                                        stats_tx
                                            .bytes_sent
                                            .fetch_add(frame.len() as u64, Ordering::Relaxed);
                                    }
                                    None => break,
                                }
                            }
                        });

                        // Task: Receive inbound packets from CDN Edge
                        let stats_rx = stats.clone();
                        let in_tx = inbound_tx.clone();
                        let mut recv_buf = Vec::with_capacity(65536);
                        let mut temp = [0u8; 8192];

                        loop {
                            match reader.read(&mut temp).await {
                                Ok(0) => break,
                                Ok(n) => {
                                    recv_buf.extend_from_slice(&temp[..n]);
                                    while let Some((from_node, packet, consumed)) =
                                        Self::decode_frame(&recv_buf)
                                    {
                                        stats_rx.packets_received.fetch_add(1, Ordering::Relaxed);
                                        stats_rx
                                            .bytes_received
                                            .fetch_add(consumed as u64, Ordering::Relaxed);
                                        let _ = in_tx.send((from_node, packet)).await;
                                        recv_buf.drain(..consumed);
                                    }
                                }
                                Err(_) => break,
                            }
                        }

                        send_task.abort();
                        connected.store(false, Ordering::Relaxed);
                        warn!("CDN Edge connection dropped. Reconnecting in 2 seconds...");
                        tokio::time::sleep(Duration::from_secs(2)).await;
                    }
                    Err(e) => {
                        warn!(
                            "Failed to connect to CDN edge {}: {}. Retrying...",
                            config.edge_addr, e
                        );
                        tokio::time::sleep(Duration::from_secs(3)).await;
                    }
                }
            }
        });
    }

    pub fn is_connected(&self) -> bool {
        self.is_connected.load(Ordering::Relaxed)
    }

    pub fn stats(&self) -> &Arc<CdnTunnelStats> {
        &self.stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    use oxide_core::PacketType;

    #[test]
    fn test_cdn_edge_frame_roundtrip() {
        let node = NodeId::new();
        let payload = Bytes::from_static(b"HELLO_IRAN_CDN");
        let packet = WirePacket::new(PacketType::Ipv4, 100, payload.clone()).unwrap();

        let encoded = CdnEdgeTunnelClient::encode_frame(node, &packet);
        assert!(encoded.len() > 24);

        let (target, decoded_packet, consumed) =
            CdnEdgeTunnelClient::decode_frame(&encoded).unwrap();
        assert_eq!(target, node);
        assert_eq!(decoded_packet.payload, payload);
        assert_eq!(consumed, encoded.len());
    }

    #[test]
    fn test_ws_upgrade_request_format() {
        let client = CdnEdgeTunnelClient::new(
            CdnEdgeConfig {
                edge_domain: "arvan.oxide.ir".to_string(),
                custom_host_header: Some("edge.parspack.com".to_string()),
                ..Default::default()
            },
            NodeId::new(),
        );

        let req_bytes = client.build_ws_upgrade_request();
        let req_str = String::from_utf8_lossy(&req_bytes);

        assert!(req_str.contains("Host: edge.parspack.com"));
        assert!(req_str.contains("Upgrade: websocket"));
        assert!(req_str.contains("Connection: Upgrade"));
    }
}
