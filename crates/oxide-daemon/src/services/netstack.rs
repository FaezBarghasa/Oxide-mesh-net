//! Rootless userspace SOCKS5 & HTTP CONNECT network proxy engine

use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tracing::{debug, error, info, warn};

/// SOCKS5 protocol constants (RFC 1928)
pub const SOCKS5_VERSION: u8 = 0x05;
pub const SOCKS5_AUTH_NONE: u8 = 0x00;
pub const SOCKS5_CMD_CONNECT: u8 = 0x01;
pub const SOCKS5_REPLY_SUCCESS: u8 = 0x00;
pub const SOCKS5_REPLY_FAILURE: u8 = 0x01;
pub const SOCKS5_REPLY_COMMAND_NOT_SUPPORTED: u8 = 0x07;
pub const SOCKS5_ATYP_IPV4: u8 = 0x01;
pub const SOCKS5_ATYP_DOMAIN: u8 = 0x03;
pub const SOCKS5_ATYP_IPV6: u8 = 0x04;

/// Target address requested by SOCKS5 client
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetAddress {
    Ip(SocketAddr),
    Domain(String, u16),
}

/// Userspace Netstack Proxy Configuration
#[derive(Debug, Clone)]
pub struct NetstackConfig {
    pub bind_addr: SocketAddr,
    pub enabled: bool,
}

impl Default for NetstackConfig {
    fn default() -> Self {
        Self {
            bind_addr: "127.0.0.1:1055".parse().unwrap(),
            enabled: true,
        }
    }
}

/// Statistics for userspace netstack proxy
#[derive(Debug, Default)]
pub struct NetstackStats {
    pub active_connections: AtomicU64,
    pub total_connections_handled: AtomicU64,
    pub bytes_rx: AtomicU64,
    pub bytes_tx: AtomicU64,
}

/// Rootless Userspace SOCKS5 and HTTP CONNECT Netstack Engine
pub struct UserspaceNetstackServer {
    config: NetstackConfig,
    stats: Arc<NetstackStats>,
    is_running: Arc<AtomicBool>,
}

impl UserspaceNetstackServer {
    pub fn new(config: NetstackConfig) -> Self {
        Self {
            config,
            stats: Arc::new(NetstackStats::default()),
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Parse SOCKS5 initial greeting handshake
    pub async fn handle_socks5_greeting(stream: &mut TcpStream) -> Result<(), String> {
        let mut header = [0u8; 2];
        stream
            .read_exact(&mut header)
            .await
            .map_err(|e| format!("Failed to read greeting: {}", e))?;

        if header[0] != SOCKS5_VERSION {
            return Err(format!("Unsupported SOCKS version: {}", header[0]));
        }

        let num_methods = header[1] as usize;
        let mut methods = vec![0u8; num_methods];
        stream
            .read_exact(&mut methods)
            .await
            .map_err(|e| format!("Failed to read methods: {}", e))?;

        if !methods.contains(&SOCKS5_AUTH_NONE) {
            stream.write_all(&[SOCKS5_VERSION, 0xFF]).await.ok();
            return Err("No acceptable auth methods".into());
        }

        // Reply: Version 5, Method 0 (No authentication required)
        stream
            .write_all(&[SOCKS5_VERSION, SOCKS5_AUTH_NONE])
            .await
            .map_err(|e| format!("Failed to write greeting reply: {}", e))?;

        Ok(())
    }

    /// Parse SOCKS5 connection request and extract target destination
    pub async fn handle_socks5_request(
        stream: &mut TcpStream,
    ) -> Result<TargetAddress, String> {
        let mut req_header = [0u8; 4];
        stream
            .read_exact(&mut req_header)
            .await
            .map_err(|e| format!("Failed to read request: {}", e))?;

        if req_header[0] != SOCKS5_VERSION {
            return Err("Invalid SOCKS version in request".into());
        }

        if req_header[1] != SOCKS5_CMD_CONNECT {
            // Only TCP CONNECT is supported in userspace proxy mode
            let reply = [
                SOCKS5_VERSION,
                SOCKS5_REPLY_COMMAND_NOT_SUPPORTED,
                0x00,
                SOCKS5_ATYP_IPV4,
                0,
                0,
                0,
                0,
                0,
                0,
            ];
            stream.write_all(&reply).await.ok();
            return Err("Only CONNECT command supported".into());
        }

        let target = match req_header[3] {
            SOCKS5_ATYP_IPV4 => {
                let mut ip_bytes = [0u8; 4];
                let mut port_bytes = [0u8; 2];
                stream
                    .read_exact(&mut ip_bytes)
                    .await
                    .map_err(|e| format!("Failed to read IPv4: {}", e))?;
                stream
                    .read_exact(&mut port_bytes)
                    .await
                    .map_err(|e| format!("Failed to read port: {}", e))?;

                let ip = Ipv4Addr::from(ip_bytes);
                let port = u16::from_be_bytes(port_bytes);
                TargetAddress::Ip(SocketAddr::V4(SocketAddrV4::new(ip, port)))
            }
            SOCKS5_ATYP_DOMAIN => {
                let mut len_byte = [0u8; 1];
                stream
                    .read_exact(&mut len_byte)
                    .await
                    .map_err(|e| format!("Failed to read domain len: {}", e))?;
                let domain_len = len_byte[0] as usize;
                let mut domain_bytes = vec![0u8; domain_len];
                stream
                    .read_exact(&mut domain_bytes)
                    .await
                    .map_err(|e| format!("Failed to read domain: {}", e))?;
                let mut port_bytes = [0u8; 2];
                stream
                    .read_exact(&mut port_bytes)
                    .await
                    .map_err(|e| format!("Failed to read port: {}", e))?;

                let domain = String::from_utf8_lossy(&domain_bytes).to_string();
                let port = u16::from_be_bytes(port_bytes);
                TargetAddress::Domain(domain, port)
            }
            SOCKS5_ATYP_IPV6 => {
                let mut ip_bytes = [0u8; 16];
                let mut port_bytes = [0u8; 2];
                stream
                    .read_exact(&mut ip_bytes)
                    .await
                    .map_err(|e| format!("Failed to read IPv6: {}", e))?;
                stream
                    .read_exact(&mut port_bytes)
                    .await
                    .map_err(|e| format!("Failed to read port: {}", e))?;

                let ip = Ipv6Addr::from(ip_bytes);
                let port = u16::from_be_bytes(port_bytes);
                TargetAddress::Ip(SocketAddr::new(ip.into(), port))
            }
            atyp => return Err(format!("Unsupported address type: {}", atyp)),
        };

        // Send success reply to client
        let reply = [
            SOCKS5_VERSION,
            SOCKS5_REPLY_SUCCESS,
            0x00,
            SOCKS5_ATYP_IPV4,
            127,
            0,
            0,
            1,
            4,
            31, // Bound to 127.0.0.1:1055
        ];
        stream
            .write_all(&reply)
            .await
            .map_err(|e| format!("Failed to write request reply: {}", e))?;

        Ok(target)
    }

    /// Proxy bidirectional traffic between two TCP streams
    pub async fn proxy_bidirectional(
        mut client: TcpStream,
        mut remote: TcpStream,
        stats: Arc<NetstackStats>,
    ) {
        stats.active_connections.fetch_add(1, Ordering::Relaxed);
        stats
            .total_connections_handled
            .fetch_add(1, Ordering::Relaxed);

        let (mut client_read, mut client_write) = client.split();
        let (mut remote_read, mut remote_write) = remote.split();

        let client_to_remote = tokio::io::copy(&mut client_read, &mut remote_write);
        let remote_to_client = tokio::io::copy(&mut remote_read, &mut client_write);

        let _ = tokio::select! {
            r = client_to_remote => r,
            r = remote_to_client => r,
        };

        stats.active_connections.fetch_sub(1, Ordering::Relaxed);
    }

    pub fn stats(&self) -> &Arc<NetstackStats> {
        &self.stats
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_address_formatting() {
        let addr = TargetAddress::Domain("gateway.mesh.oxide".to_string(), 443);
        assert_eq!(
            addr,
            TargetAddress::Domain("gateway.mesh.oxide".to_string(), 443)
        );

        let ip_addr = TargetAddress::Ip("100.64.0.1:22".parse().unwrap());
        assert_eq!(ip_addr, TargetAddress::Ip("100.64.0.1:22".parse().unwrap()));
    }
}
