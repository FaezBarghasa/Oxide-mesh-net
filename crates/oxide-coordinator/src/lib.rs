//! Embedded MQTT broker and Actix Web coordinator for oxide-mesh-net

pub mod auth;
pub mod cluster;
pub mod config;
pub mod engine;
pub mod error;
pub mod handlers;
pub mod storage;

pub use cluster::{ClusterConfig, ClusterEngine, ClusterLogEntry, ClusterRole, ReplicatedState};
pub use config::{CoordinatorConfig, MqttListenerConfig, HttpConfig, StorageConfig, StorageBackendType, AuthConfig, OidcConfig};
pub use engine::{Coordinator, AppState};
pub use error::{CoordinatorError, Result};