//! MagicDNS error types

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DnsError {
    #[error("DNS protocol error: {0}")]
    Protocol(String),

    #[error("Zone not found: {0}")]
    ZoneNotFound(String),

    #[error("Record not found: {0}")]
    RecordNotFound(String),

    #[error("Upstream resolver error: {0}")]
    UpstreamError(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, DnsError>;
