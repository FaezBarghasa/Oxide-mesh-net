//! TUN interface error types

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TunError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Interface not found: {0}")]
    InterfaceNotFound(String),

    #[error("Interface already exists: {0}")]
    InterfaceExists(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Queue creation failed: {0}")]
    QueueCreationFailed(String),

    #[error("Platform not supported: {0}")]
    PlatformNotSupported(String),

    #[error("Multi-queue not supported: {0}")]
    MultiQueueNotSupported(String),

    #[error("Device busy: {0}")]
    DeviceBusy(String),

    #[error("MTU too large: {0}")]
    MtuTooLarge(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, TunError>;
