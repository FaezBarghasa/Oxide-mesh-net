//! Async IPC Client for interacting with the oxide-daemon

use super::protocol::{
    DaemonStatusDto, IpcRequest, IpcResponse, PeerStatusDto, RouteEntryDto, read_frame, write_frame,
};
use super::socket::{DEFAULT_SOCKET_PATH, FALLBACK_SOCKET_PATH};
use std::io::{Error, ErrorKind};
use std::path::Path;
use tokio::net::UnixStream;

/// High-performance async IPC client for oxide-daemon
pub struct IpcClient {
    stream: UnixStream,
}

impl IpcClient {
    /// Connect to daemon Unix domain socket (with auto-fallback)
    pub async fn connect(custom_path: Option<&Path>) -> Result<Self, std::io::Error> {
        let stream = if let Some(path) = custom_path {
            UnixStream::connect(path).await?
        } else if Path::new(DEFAULT_SOCKET_PATH).exists() {
            match UnixStream::connect(DEFAULT_SOCKET_PATH).await {
                Ok(s) => s,
                Err(_) => UnixStream::connect(FALLBACK_SOCKET_PATH).await?,
            }
        } else {
            UnixStream::connect(FALLBACK_SOCKET_PATH).await?
        };

        Ok(Self { stream })
    }

    /// Send a request and wait for a response
    pub async fn send(&mut self, req: &IpcRequest) -> Result<IpcResponse, std::io::Error> {
        write_frame(&mut self.stream, req).await?;
        read_frame(&mut self.stream).await
    }

    /// Query daemon status
    pub async fn status(&mut self) -> Result<DaemonStatusDto, std::io::Error> {
        match self.send(&IpcRequest::Status).await? {
            IpcResponse::Status(dto) => Ok(dto),
            IpcResponse::Error { error } => Err(Error::other(error)),
            _ => Err(Error::new(ErrorKind::InvalidData, "Unexpected response")),
        }
    }

    /// Bring network up
    pub async fn up(&mut self, config_path: Option<String>) -> Result<String, std::io::Error> {
        match self.send(&IpcRequest::Up { config_path }).await? {
            IpcResponse::Success { message } => Ok(message),
            IpcResponse::Error { error } => Err(Error::other(error)),
            _ => Err(Error::new(ErrorKind::InvalidData, "Unexpected response")),
        }
    }

    /// Bring network down
    pub async fn down(&mut self) -> Result<String, std::io::Error> {
        match self.send(&IpcRequest::Down).await? {
            IpcResponse::Success { message } => Ok(message),
            IpcResponse::Error { error } => Err(Error::other(error)),
            _ => Err(Error::new(ErrorKind::InvalidData, "Unexpected response")),
        }
    }

    /// Query active routing table
    pub async fn routes(&mut self) -> Result<Vec<RouteEntryDto>, std::io::Error> {
        match self.send(&IpcRequest::Routes).await? {
            IpcResponse::Routes(routes) => Ok(routes),
            IpcResponse::Error { error } => Err(Error::other(error)),
            _ => Err(Error::new(ErrorKind::InvalidData, "Unexpected response")),
        }
    }

    /// Query connected peers
    pub async fn peers(&mut self) -> Result<Vec<PeerStatusDto>, std::io::Error> {
        match self.send(&IpcRequest::Peers).await? {
            IpcResponse::Peers(peers) => Ok(peers),
            IpcResponse::Error { error } => Err(Error::other(error)),
            _ => Err(Error::new(ErrorKind::InvalidData, "Unexpected response")),
        }
    }

    /// Reload ACL rules dynamically
    pub async fn acl_reload(
        &mut self,
        rules_json: Option<String>,
    ) -> Result<String, std::io::Error> {
        match self.send(&IpcRequest::AclReload { rules_json }).await? {
            IpcResponse::Success { message } => Ok(message),
            IpcResponse::Error { error } => Err(Error::other(error)),
            _ => Err(Error::new(ErrorKind::InvalidData, "Unexpected response")),
        }
    }

    /// Ping daemon
    pub async fn ping(&mut self) -> Result<bool, std::io::Error> {
        match self.send(&IpcRequest::Ping).await? {
            IpcResponse::Pong => Ok(true),
            _ => Ok(false),
        }
    }
}
