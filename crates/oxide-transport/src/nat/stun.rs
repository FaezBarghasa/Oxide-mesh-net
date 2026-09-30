//! RFC 5389 / RFC 8489 STUN Client and NAT Traversal Protocol

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::time::Duration;
use tokio::net::UdpSocket;

/// STUN Magic Cookie (RFC 5389)
pub const STUN_MAGIC_COOKIE: u32 = 0x2112A442;
/// STUN Binding Request Message Type
pub const BINDING_REQUEST: u16 = 0x0001;
/// STUN Binding Response Message Type
pub const BINDING_RESPONSE: u16 = 0x0101;
/// STUN XOR-MAPPED-ADDRESS Attribute Type
pub const ATTR_XOR_MAPPED_ADDRESS: u16 = 0x0020;
/// STUN MAPPED-ADDRESS Attribute Type
pub const ATTR_MAPPED_ADDRESS: u16 = 0x0001;

/// Detected NAT Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NatType {
    OpenInternet,
    FullCone,
    RestrictedCone,
    PortRestrictedCone,
    Symmetric,
    UdpBlocked,
}

/// STUN Client for reflexive endpoint discovery
#[derive(Debug, Clone)]
pub struct StunClient {
    pub stun_servers: Vec<String>,
    pub timeout: Duration,
}

impl Default for StunClient {
    fn default() -> Self {
        Self {
            stun_servers: vec![
                "stun.l.google.com:19302".to_string(),
                "stun1.l.google.com:19302".to_string(),
                "stun.cloudflare.com:3478".to_string(),
            ],
            timeout: Duration::from_millis(1500),
        }
    }
}

impl StunClient {
    pub fn new(stun_servers: Vec<String>) -> Self {
        Self {
            stun_servers,
            timeout: Duration::from_millis(1500),
        }
    }

    /// Build a 20-byte STUN Binding Request with a 12-byte transaction ID
    pub fn build_binding_request(transaction_id: &[u8; 12]) -> [u8; 20] {
        let mut req = [0u8; 20];
        // Message Type: 0x0001 (Binding Request)
        req[0] = 0x00;
        req[1] = 0x01;
        // Message Length: 0x0000 (No attributes)
        req[2] = 0x00;
        req[3] = 0x00;
        // Magic Cookie: 0x2112A442
        req[4..8].copy_from_slice(&STUN_MAGIC_COOKIE.to_be_bytes());
        // Transaction ID: 12 bytes
        req[8..20].copy_from_slice(transaction_id);
        req
    }

    /// Parse a STUN Binding Response to extract the mapped public SocketAddr
    pub fn parse_binding_response(
        buf: &[u8],
        expected_tx_id: &[u8; 12],
    ) -> Result<SocketAddr, String> {
        if buf.len() < 20 {
            return Err("STUN packet too short for header".into());
        }

        let msg_type = u16::from_be_bytes([buf[0], buf[1]]);
        if msg_type != BINDING_RESPONSE {
            return Err(format!("Unexpected STUN message type: {:#06x}", msg_type));
        }

        let magic = u32::from_be_bytes([buf[4], buf[5], buf[6], buf[7]]);
        if magic != STUN_MAGIC_COOKIE {
            return Err("Invalid STUN magic cookie".into());
        }

        if &buf[8..20] != expected_tx_id {
            return Err("STUN transaction ID mismatch".into());
        }

        let msg_len = u16::from_be_bytes([buf[2], buf[3]]) as usize;
        let mut offset = 20;
        let end = (offset + msg_len).min(buf.len());

        while offset + 4 <= end {
            let attr_type = u16::from_be_bytes([buf[offset], buf[offset + 1]]);
            let attr_len = u16::from_be_bytes([buf[offset + 2], buf[offset + 3]]) as usize;
            offset += 4;

            if offset + attr_len > end {
                break;
            }

            if attr_type == ATTR_XOR_MAPPED_ADDRESS && attr_len >= 8 {
                let family = buf[offset + 1];
                if family == 0x01 {
                    // IPv4
                    let xor_port = u16::from_be_bytes([buf[offset + 2], buf[offset + 3]]);
                    let port = xor_port ^ (STUN_MAGIC_COOKIE >> 16) as u16;

                    let xor_ip = u32::from_be_bytes([
                        buf[offset + 4],
                        buf[offset + 5],
                        buf[offset + 6],
                        buf[offset + 7],
                    ]);
                    let ip = Ipv4Addr::from(xor_ip ^ STUN_MAGIC_COOKIE);
                    return Ok(SocketAddr::V4(SocketAddrV4::new(ip, port)));
                }
            } else if attr_type == ATTR_MAPPED_ADDRESS && attr_len >= 8 {
                let family = buf[offset + 1];
                if family == 0x01 {
                    // Plain IPv4
                    let port = u16::from_be_bytes([buf[offset + 2], buf[offset + 3]]);
                    let ip = Ipv4Addr::new(
                        buf[offset + 4],
                        buf[offset + 5],
                        buf[offset + 6],
                        buf[offset + 7],
                    );
                    return Ok(SocketAddr::V4(SocketAddrV4::new(ip, port)));
                }
            }

            // STUN attributes are padded to 4-byte boundaries
            let padding = (4 - (attr_len % 4)) % 4;
            offset += attr_len + padding;
        }

        Err("No Mapped Address attribute found in STUN response".into())
    }

    /// Query a STUN server via an existing UDP socket to discover reflexive endpoint
    pub async fn query_server(
        &self,
        socket: &UdpSocket,
        server: &str,
    ) -> Result<SocketAddr, String> {
        let server_addr = tokio::net::lookup_host(server)
            .await
            .map_err(|e| format!("DNS lookup failed for {}: {}", server, e))?
            .next()
            .ok_or_else(|| format!("Could not resolve STUN server {}", server))?;

        let tx_id = [
            0x42, 0x18, 0x9a, 0xef, 0x33, 0x7c, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66,
        ];
        let req = Self::build_binding_request(&tx_id);

        socket
            .send_to(&req, server_addr)
            .await
            .map_err(|e| format!("Failed to send STUN request to {}: {}", server, e))?;

        let mut buf = [0u8; 1024];
        let recv_result = tokio::time::timeout(self.timeout, socket.recv_from(&mut buf)).await;

        match recv_result {
            Ok(Ok((len, _from))) => Self::parse_binding_response(&buf[..len], &tx_id),
            Ok(Err(e)) => Err(format!("Socket recv error: {}", e)),
            Err(_) => Err(format!("STUN request to {} timed out", server)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stun_binding_request_and_response_parsing() {
        let tx_id = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let req = StunClient::build_binding_request(&tx_id);
        assert_eq!(req.len(), 20);
        assert_eq!(u16::from_be_bytes([req[0], req[1]]), BINDING_REQUEST);

        // Construct synthetic XOR-MAPPED-ADDRESS response for 198.51.100.42:51820
        let target_ip = Ipv4Addr::new(198, 51, 100, 42);
        let target_port: u16 = 51820;

        let xor_port = target_port ^ (STUN_MAGIC_COOKIE >> 16) as u16;
        let xor_ip = u32::from(target_ip) ^ STUN_MAGIC_COOKIE;

        let mut resp = Vec::new();
        // Header (20 bytes)
        resp.extend_from_slice(&BINDING_RESPONSE.to_be_bytes());
        resp.extend_from_slice(&12u16.to_be_bytes()); // Attribute len
        resp.extend_from_slice(&STUN_MAGIC_COOKIE.to_be_bytes());
        resp.extend_from_slice(&tx_id);

        // XOR-MAPPED-ADDRESS Attribute (8 bytes data + 4 bytes header = 12)
        resp.extend_from_slice(&ATTR_XOR_MAPPED_ADDRESS.to_be_bytes());
        resp.extend_from_slice(&8u16.to_be_bytes());
        resp.push(0x00); // Reserved
        resp.push(0x01); // IPv4
        resp.extend_from_slice(&xor_port.to_be_bytes());
        resp.extend_from_slice(&xor_ip.to_be_bytes());

        let mapped = StunClient::parse_binding_response(&resp, &tx_id).expect("parsing succeeded");
        assert_eq!(mapped.ip(), target_ip);
        assert_eq!(mapped.port(), target_port);
    }
}
