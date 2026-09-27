//! Transport error types

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransportError {
    #[error("QUIC error: {0}")]
    Quinn(#[from] quinn::ConnectionError),

    #[error("Connect error: {0}")]
    Connect(#[from] quinn::ConnectError),

    #[error("Datagram send error: {0}")]
    SendDatagram(#[from] quinn::SendDatagramError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Connection lost: {0}")]
    ConnectionLost(String),

    #[error("Datagram send failed: {0}")]
    DatagramSendFailed(String),

    #[error("Stream error: {0}")]
    StreamError(String),

    #[error("Authentication failed: {0}")]
    AuthFailed(String),

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Not connected")]
    NotConnected,

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Channel error: {0}")]
    Channel(String),
}

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for TransportError {
    fn from(err: tokio::sync::mpsc::error::SendError<T>) -> Self {
        TransportError::Channel(err.to_string())
    }
}

impl From<tokio::sync::oneshot::error::RecvError> for TransportError {
    fn from(err: tokio::sync::oneshot::error::RecvError) -> Self {
        TransportError::Channel(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, TransportError>;
