//! Core error types for oxide-mesh-net

use std::{io, net::AddrParseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OxideError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Address parse error: {0}")]
    AddrParse(#[from] AddrParseError),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Deserialization error: {0}")]
    Deserialization(String),

    #[error("Cryptographic error: {0}")]
    Crypto(String),

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Network interface error: {0}")]
    NetworkInterface(String),

    #[error("Transport error: {0}")]
    Transport(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Authorization error: {0}")]
    Authz(String),

    #[error("NAT traversal error: {0}")]
    NatTraversal(String),

    #[error("DNS error: {0}")]
    Dns(String),

    #[error("ACL error: {0}")]
    Acl(String),

    #[error("Coordinator error: {0}")]
    Coordinator(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("Resource exhausted: {0}")]
    ResourceExhausted(String),

    #[error("Operation timed out: {0}")]
    Timeout(String),

    #[error("Not implemented: {0}")]
    NotImplemented(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, OxideError>;

#[macro_export]
macro_rules! oxide_error {
    ($variant:ident, $($arg:tt)*) => {
        $crate::OxideError::$variant(format!($($arg)*))
    };
}