//! QUIC transport configuration

use std::time::Duration;
use quinn::{ClientConfig, ServerConfig, TransportConfig, VarInt};
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
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
    pub fn configure(&self, transport: &mut TransportConfig) {
        match self {
            CongestionControl::Cubic => {
                // Cubic is default in quinn
            }
            CongestionControl::Bbr => {
                // BBR configuration would go here
                // quinn doesn't expose BBR directly yet, but we can set parameters
            }
            CongestionControl::NewReno => {
                // NewReno configuration
            }
        }
    }
}

/// Build client configuration
pub fn make_client_config(config: &TransportConfig) -> Result<ClientConfig, TransportError> {
    let mut root_store = rustls::RootCertStore::empty();
    for cert in &config.root_certs {
        root_store.add(cert.clone())?;
    }

    let mut client_crypto = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    client_crypto.alpn_protocols = config.alpn.clone();

    let mut transport = TransportConfig::default();
    transport.max_idle_timeout(Some(config.idle_timeout.try_into().map_err(|e| TransportError::Config(e.to_string()))?));
    transport.keep_alive_interval(Some(config.keepalive_interval));
    transport.max_datagram_frame_size(config.max_datagram_size as u64);
    
    if config.enable_0rtt {
        transport.enable_0rtt(true);
    }

    Ok(ClientConfig::new(client_crypto.into()))
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

    let mut transport = TransportConfig::default();
    transport.max_idle_timeout(Some(config.idle_timeout.try_into().map_err(|e| TransportError::Config(e.to_string()))?));
    transport.keep_alive_interval(Some(config.keepalive_interval));
    transport.max_datagram_frame_size(config.max_datagram_size as u64);
    transport.max_concurrent_bidi_streams(VarInt::from_u32(100));
    transport.max_concurrent_uni_streams(VarInt::from_u32(1000));
    
    if config.enable_0rtt {
        transport.enable_0rtt(true);
    }

    Ok(ServerConfig::with_crypto(server_crypto.into()))
}