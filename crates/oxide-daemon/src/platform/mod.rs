//! Platform-specific networking and policy routing modules

pub mod linux_routing;

pub use linux_routing::{
    DEFAULT_FWMARK, DEFAULT_ROUTING_TABLE, PolicyRoutingConfig, PolicyRoutingManager,
};
