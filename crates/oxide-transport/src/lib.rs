//! QUIC transport engine for oxide-mesh-net

pub mod circuit_breaker;
pub mod config;
pub mod dplpmtud;
pub mod engine;
pub mod error;
pub mod nat;
pub mod porthopper;
pub mod routing;

pub use circuit_breaker::{CircuitBreakerConfig, CircuitStatus, TransportCircuitBreaker};
pub use config::{CongestionControl, TransportConfig, make_client_config, make_server_config};
pub use dplpmtud::{DplpmtudConfig, DplpmtudEngine, DplpmtudPhase};
pub use engine::{TransportEngine, TransportEvent, TransportHandle, TransportStats};
pub use error::{Result, TransportError};
pub use nat::{EndpointCandidate, MagicsockEngine, NatType, PathType, StunClient, UpnpClient};
pub use porthopper::{PortHopper, PortHopperConfig};
pub use routing::{L1DirectMappedCache, RadixRoutingTable, RcuRouter, RouteEntry, RouteTarget};
