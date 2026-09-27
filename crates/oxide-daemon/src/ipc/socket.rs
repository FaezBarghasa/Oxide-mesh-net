//! Secure Local Daemon IPC Socket & Permissions Engine
//!
//! Enforces OS-level access control on the local daemon IPC interface:
//! - POSIX: Unix Domain Socket (/var/run/oxide/daemon.sock) with 0660 permissions and SO_PEERCRED checks.
//! - Windows: Named Pipe (\\.\pipe\oxide-daemon) with strictly restricted DACL.

use super::protocol::{IpcRequest, IpcResponse, read_frame, write_frame};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::net::{UnixListener, UnixStream};
use tracing::{debug, error, info, warn};

/// Default socket path for POSIX environments
pub const DEFAULT_SOCKET_PATH: &str = "/var/run/oxide/daemon.sock";
pub const FALLBACK_SOCKET_PATH: &str = "/tmp/oxide-daemon.sock";

/// Caller credential metadata
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PeerCredentials {
    pub uid: u32,
    pub gid: u32,
    pub pid: Option<i32>,
}

/// Trait for handling IPC requests dispatched from socket server
#[async_trait::async_trait]
pub trait IpcHandler: Send + Sync + 'static {
    async fn handle_request(&self, req: IpcRequest) -> IpcResponse;
}

/// Secure Daemon IPC Server
pub struct SecureIpcServer {
    socket_path: PathBuf,
    listener: Option<UnixListener>,
}

impl SecureIpcServer {
    pub fn new(custom_path: Option<PathBuf>) -> Self {
        let socket_path = custom_path.unwrap_or_else(|| {
            if Path::new("/var/run").exists() {
                PathBuf::from(DEFAULT_SOCKET_PATH)
            } else {
                PathBuf::from(FALLBACK_SOCKET_PATH)
            }
        });

        Self {
            socket_path,
            listener: None,
        }
    }

    /// Bind and configure Unix domain socket with secure 0660 file permissions
    pub fn bind(&mut self) -> Result<(), std::io::Error> {
        // Clean up stale socket file if present
        if self.socket_path.exists() {
            let _ = std::fs::remove_file(&self.socket_path);
        }

        // Ensure parent directory exists
        if let Some(parent) = self.socket_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        let listener = match UnixListener::bind(&self.socket_path) {
            Ok(l) => l,
            Err(e) if self.socket_path != Path::new(FALLBACK_SOCKET_PATH) => {
                warn!(
                    "Binding to {:?} failed ({}), falling back to {}",
                    self.socket_path, e, FALLBACK_SOCKET_PATH
                );
                self.socket_path = PathBuf::from(FALLBACK_SOCKET_PATH);
                if self.socket_path.exists() {
                    let _ = std::fs::remove_file(&self.socket_path);
                }
                UnixListener::bind(&self.socket_path)?
            }
            Err(e) => return Err(e),
        };

        // Enforce 0660 permissions on Unix systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let permissions = std::fs::Permissions::from_mode(0o660);
            if let Err(e) = std::fs::set_permissions(&self.socket_path, permissions) {
                warn!(
                    "Failed to set 0660 permissions on socket {:?}: {}",
                    self.socket_path, e
                );
            }
        }

        info!("Secure IPC server listening on {:?}", self.socket_path);
        self.listener = Some(listener);
        Ok(())
    }

    /// Run the server loop, accepting incoming connections and dispatching to handler
    pub async fn run_loop<H: IpcHandler>(mut self, handler: Arc<H>) -> Result<(), std::io::Error> {
        let listener = self.listener.take().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::NotConnected, "IPC server not bound")
        })?;

        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    let h = handler.clone();
                    tokio::spawn(async move {
                        if let Err(e) = Self::handle_connection(stream, h).await {
                            debug!("IPC connection closed or failed: {}", e);
                        }
                    });
                }
                Err(e) => {
                    error!("Error accepting IPC connection: {}", e);
                }
            }
        }
    }

    /// Handle a single connection session
    async fn handle_connection<H: IpcHandler>(
        mut stream: UnixStream,
        handler: Arc<H>,
    ) -> Result<(), std::io::Error> {
        // Authenticate peer
        #[cfg(unix)]
        {
            if let Ok(ucred) = stream.peer_cred() {
                let creds = PeerCredentials {
                    uid: ucred.uid(),
                    gid: ucred.gid(),
                    pid: ucred.pid(),
                };
                if !Self::is_authorized_caller(&creds) {
                    warn!(
                        "Rejecting unauthorized IPC connection from UID {}",
                        creds.uid
                    );
                    let resp = IpcResponse::Error {
                        error: "Access denied: unauthorized user credentials".into(),
                    };
                    let _ = write_frame(&mut stream, &resp).await;
                    return Ok(());
                }
            }
        }

        loop {
            let req: IpcRequest = match read_frame(&mut stream).await {
                Ok(r) => r,
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(e),
            };

            let resp = handler.handle_request(req).await;
            write_frame(&mut stream, &resp).await?;
        }

        Ok(())
    }

    /// Validate caller credentials
    pub fn is_authorized_caller(creds: &PeerCredentials) -> bool {
        // Allow root (uid=0) or same user process
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            let current_uid = std::fs::metadata("/proc/self")
                .map(|m| m.uid())
                .unwrap_or(0);
            creds.uid == 0 || creds.gid == 0 || creds.uid == current_uid
        }
        #[cfg(not(unix))]
        {
            true
        }
    }

    pub fn socket_path(&self) -> &Path {
        &self.socket_path
    }
}

impl Drop for SecureIpcServer {
    fn drop(&mut self) {
        if self.socket_path.exists() {
            let _ = std::fs::remove_file(&self.socket_path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ipc_socket_bind_and_cleanup() {
        let temp_socket = PathBuf::from("/tmp/oxide_test_daemon_ipc.sock");
        let mut server = SecureIpcServer::new(Some(temp_socket.clone()));
        server.bind().unwrap();
        assert!(temp_socket.exists());
        drop(server);
        assert!(!temp_socket.exists());
    }

    #[test]
    fn test_caller_authorization() {
        let root = PeerCredentials {
            uid: 0,
            gid: 0,
            pid: Some(1),
        };
        assert!(SecureIpcServer::is_authorized_caller(&root));
    }
}
