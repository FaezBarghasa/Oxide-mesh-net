//! QUIC transport configuration

use std::sync::Arc;
use std::time::Duration;
use quinn::{ClientConfig, ServerConfig, TransportConfig as QuinnTransportConfig, VarInt};
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use quinn::crypto::rustls::{QuicClientConfig, QuicServerConfig};
use crate::error::TransportError;

/// Transport configuration
#[derive(Debug, Clone)]
pub struct TransportConfig {
    /// Local bind addresses for UDP sockets
    pub bind_addrs: Vec<std::net::SocketAddr>,
    /// Maximum concurrent connections
    pub max_connections: usize,
    /// Idle timeout
    pub idle_timeout: Duration,
    /// Keepalive interval
    pub keepalive_interval: Duration,
    /// Maximum datagram frame size
    pub max_datagram_size: usize,
    /// Enable 0-RTT
    pub enable_0rtt: bool,
    /// Congestion control algorithm
    pub congestion_control: CongestionControl,
    /// Server TLS certificate (for listeners)
    pub server_cert: Option<CertificateDer<'static>>,
    /// Server TLS private key (for listeners)
    pub server_key: Option<PrivateKeyDer<'static>>,
    /// Client root CA certificates
    pub root_certs: Vec<CertificateDer<'static>>,
    /// ALPN protocols
    pub alpn: Vec<Vec<u8>>,
}

impl Default for TransportConfig {
    fn default() -> Self {
        Self {
            bind_addrs: vec!["0.0.0.0:0".parse().unwrap(), "[::]:0".parse().unwrap()],
            max_connections: 10000,
            idle_timeout: Duration::from_secs(30),
            keepalive_interval: Duration::from_secs(10),
            max_datagram_size: 65536,
            enable_0rtt: true,
            congestion_control: CongestionControl::Bbr,
            server_cert: None,
            server_key: None,
            root_certs: Vec::new(),
            alpn: vec![b"oxide-mesh/1".to_vec()],
        }
    }
}

/// Congestion control algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CongestionControl {
    Cubic,
    Bbr,
    NewReno,
}

impl CongestionControl {
    pub fn configure(&self, _transport: &mut QuinnTransportConfig) {
        match self {
            CongestionControl::Cubic => {}
            CongestionControl::Bbr => {}
            CongestionControl::NewReno => {}
        }
    }
}

/// Build client configuration
pub fn make_client_config(config: &TransportConfig) -> Result<ClientConfig, TransportError> {
    let mut root_store = rustls::RootCertStore::empty();
    for cert in &config.root_certs {
        root_store.add(cert.clone()).map_err(|e| TransportError::Config(e.to_string()))?;
    }

    let mut client_crypto = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    client_crypto.alpn_protocols = config.alpn.clone();

    let mut transport = QuinnTransportConfig::default();
    if let Ok(idle) = config.idle_timeout.try_into() {
        transport.max_idle_timeout(Some(idle));
    }
    transport.keep_alive_interval(Some(config.keepalive_interval));
    transport.datagram_receive_buffer_size(Some(config.max_datagram_size));
    transport.datagram_send_buffer_size(config.max_datagram_size);

    let quic_crypto = QuicClientConfig::try_from(client_crypto)
        .map_err(|e| TransportError::Config(e.to_string()))?;
    let mut client_config = ClientConfig::new(Arc::new(quic_crypto));
    client_config.transport_config(Arc::new(transport));

    Ok(client_config)
}

/// Build server configuration
pub fn make_server_config(config: &TransportConfig) -> Result<ServerConfig, TransportError> {
    let cert = config.server_cert.as_ref().ok_or_else(|| TransportError::Config("Missing server certificate".into()))?;
    let key = config.server_key.as_ref().ok_or_else(|| TransportError::Config("Missing server key".into()))?;

    let mut server_crypto = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(vec![cert.clone()], key.clone_key())
        .map_err(|e| TransportError::Config(e.to_string()))?;

    server_crypto.alpn_protocols = config.alpn.clone();

    let mut transport = QuinnTransportConfig::default();
    if let Ok(idle) = config.idle_timeout.try_into() {
        transport.max_idle_timeout(Some(idle));
    }
    transport.keep_alive_interval(Some(config.keepalive_interval));
    transport.datagram_receive_buffer_size(Some(config.max_datagram_size));
    transport.datagram_send_buffer_size(config.max_datagram_size);
    transport.max_concurrent_bidi_streams(VarInt::from_u32(100));
    transport.max_concurrent_uni_streams(VarInt::from_u32(1000));

    let quic_crypto = QuicServerConfig::try_from(server_crypto)
        .map_err(|e| TransportError::Config(e.to_string()))?;
    let mut server_config = ServerConfig::with_crypto(Arc::new(quic_crypto));
    server_config.transport_config(Arc::new(transport));

    Ok(server_config)
}