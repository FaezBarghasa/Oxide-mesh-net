//! MagicDNS and split-DNS engine for oxide-mesh-net

pub mod engine;
pub mod error;

pub use engine::{DnsConfig, MagicDns, MagicDnsRequestHandler, SplitHorizonResolver, DnsStats};
pub use error::{DnsError, Result};