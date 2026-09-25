//! MagicDNS and Split-DNS engine for oxide-mesh-net

pub mod error {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum DnsError {
        #[error("DNS resolution error: {0}")]
        Resolution(String),
        #[error("IO error: {0}")]
        Io(#[from] std::io::Error),
    }

    pub type Result<T> = std::result::Result<T, DnsError>;
}

pub use error::{DnsError, Result};