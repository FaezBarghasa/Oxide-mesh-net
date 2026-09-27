//! High-performance background daemon and Actix Web administrative server for oxide-mesh-net

pub mod config;
pub mod engine;
pub mod ipc;
pub mod services;

pub use config::{DaemonConfig, StaticRouteConfig};
pub use engine::DaemonEngine;
pub use ipc::{
    client::IpcClient,
    protocol::{DaemonStatusDto, IpcRequest, IpcResponse, PeerStatusDto, RouteEntryDto},
    socket::{IpcHandler, PeerCredentials, SecureIpcServer},
};
