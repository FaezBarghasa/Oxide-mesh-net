//! Embedded MQTT broker and Actix Web coordinator for oxide-mesh-net

pub mod config;
pub mod engine;
pub mod error;
pub mod auth;
pub mod storage;
pub mod handlers;

pub use config::{CoordinatorConfig, MqttListenerConfig, HttpConfig, StorageConfig, StorageBackendType, AuthConfig, OidcConfig};
pub use engine::{Coordinator, AppState};
pub use error::{CoordinatorError, Result};