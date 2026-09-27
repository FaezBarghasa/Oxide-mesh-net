//! Strongly typed IPC message protocol and framing for oxide-daemon

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::io::{Error, ErrorKind};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Maximum allowed IPC frame size (16 MB guard)
pub const MAX_IPC_FRAME_SIZE: usize = 16 * 1024 * 1024;

/// Request messages sent from CLI to Daemon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IpcRequest {
    Status,
    Up { config_path: Option<String> },
    Down,
    Routes,
    Peers,
    AclReload { rules_json: Option<String> },
    Ping,
}

/// Response messages returned from Daemon to CLI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IpcResponse {
    Status(DaemonStatusDto),
    Routes(Vec<RouteEntryDto>),
    Peers(Vec<PeerStatusDto>),
    Success { message: String },
    Error { error: String },
    Pong,
}

/// High-level daemon status DTO
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DaemonStatusDto {
    pub node_id: String,
    pub tun_name: String,
    pub mtu: u16,
    pub active_peers: usize,
    pub confirmed_pmtu: u16,
    pub circuit_breaker_state: String,
    pub port_hopping_epoch: u64,
    pub current_port: u16,
    pub dns_active: bool,
    pub mss_clamping_active: bool,
    pub uptime_secs: u64,
}

/// Routing entry DTO
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RouteEntryDto {
    pub prefix: String,
    pub target_node: String,
    pub pmtu: u16,
    pub is_direct: bool,
}

/// Peer connection state DTO
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PeerStatusDto {
    pub node_id: String,
    pub overlay_ip: String,
    pub endpoint: Option<String>,
    pub pmtu: u16,
    pub rtt_ms: Option<f64>,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub last_seen_secs: u64,
}

/// Write a length-prefixed JSON frame asynchronously
pub async fn write_frame<W: AsyncWriteExt + Unpin, T: Serialize>(
    writer: &mut W,
    msg: &T,
) -> std::io::Result<()> {
    let bytes = serde_json::to_vec(msg).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
    let len = bytes.len() as u32;
    writer.write_all(&len.to_be_bytes()).await?;
    writer.write_all(&bytes).await?;
    writer.flush().await?;
    Ok(())
}

/// Read a length-prefixed JSON frame asynchronously
pub async fn read_frame<R: AsyncReadExt + Unpin, T: DeserializeOwned>(
    reader: &mut R,
) -> std::io::Result<T> {
    let mut len_buf = [0u8; 4];
    reader.read_exact(&mut len_buf).await?;
    let len = u32::from_be_bytes(len_buf) as usize;
    if len > MAX_IPC_FRAME_SIZE {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!(
                "IPC frame size {} exceeds maximum {}",
                len, MAX_IPC_FRAME_SIZE
            ),
        ));
    }
    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf).await?;
    serde_json::from_slice(&buf).map_err(|e| Error::new(ErrorKind::InvalidData, e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[tokio::test]
    async fn test_ipc_framing_roundtrip() {
        let mut buffer = Vec::new();
        let req = IpcRequest::Status;
        write_frame(&mut buffer, &req).await.unwrap();

        let mut cursor = Cursor::new(buffer);
        let decoded: IpcRequest = read_frame(&mut cursor).await.unwrap();
        match decoded {
            IpcRequest::Status => {}
            _ => panic!("Decoded incorrect request type"),
        }
    }
}
