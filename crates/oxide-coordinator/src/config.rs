//! Coordinator configuration

use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;
use serde::{Deserialize, Serialize};

/// Coordinator configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinatorConfig {
    /// Mesh name
    pub mesh_name: String,
    /// MQTT listener addresses
    pub mqtt_listeners: Vec<MqttListenerConfig>,
    /// Actix Web HTTP server config
    pub http: HttpConfig,
    /// Storage configuration
    pub storage: StorageConfig,
    /// Authentication configuration
    pub auth: AuthConfig,
    /// Cluster configuration (for HA)
    pub cluster: Option<ClusterConfig>,
    /// Rate limiting
    pub rate_limit: RateLimitConfig,
    /// TLS configuration
    pub tls: Option<TlsConfig>,
}

impl Default for CoordinatorConfig {
    fn default() -> Self {
        Self {
            mesh_name: "oxide".into(),
            mqtt_listeners: vec![MqttListenerConfig::default()],
            http: HttpConfig::default(),
            storage: StorageConfig::default(),
            auth: AuthConfig::default(),
            cluster: None,
            rate_limit: RateLimitConfig::default(),
            tls: None,
        }
    }
}

/// MQTT listener configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttListenerConfig {
    /// Bind address
    pub bind: SocketAddr,
    /// Enable TLS
    pub tls: bool,
    /// TLS certificate path
    pub cert_path: Option<PathBuf>,
    /// TLS key path
    pub key_path: Option<PathBuf>,
    /// Max connections
    pub max_connections: usize,
    /// Max packet size
    pub max_packet_size: usize,
}

impl Default for MqttListenerConfig {
    fn default() -> Self {
        Self {
            bind: "0.0.0.0:1883".parse().unwrap(),
            tls: false,
            cert_path: None,
            key_path: None,
            max_connections: 10000,
            max_packet_size: 1024 * 1024, // 1MB
        }
    }
}

/// HTTP server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpConfig {
    /// Bind address
    pub bind: SocketAddr,
    /// Number of workers
    pub workers: usize,
    /// Enable TLS
    pub tls: bool,
    /// TLS certificate path
    pub cert_path: Option<PathBuf>,
    /// TLS key path
    pub key_path: Option<PathBuf>,
    /// Request timeout
    pub request_timeout: Duration,
    /// Body limit
    pub body_limit: usize,
    /// Enable CORS
    pub cors: bool,
    /// Static files directory (for Dioxus WASM)
    pub static_dir: Option<PathBuf>,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            bind: "0.0.0.0:8080".parse().unwrap(),
            workers: std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4),
            tls: false,
            cert_path: None,
            key_path: None,
            request_timeout: Duration::from_secs(30),
            body_limit: 1024 * 1024, // 1MB
            cors: true,
            static_dir: None,
        }
    }
}

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Storage backend type
    pub backend: StorageBackendType,
    /// Data directory
    pub data_dir: PathBuf,
    /// Redis URL (for ephemeral caching and distributed synchronization)
    pub redis_url: Option<String>,
    /// SurrealDB endpoint URL (e.g. ws://127.0.0.1:8000 or surrealkv://...)
    pub surreal_url: Option<String>,
    /// SurrealDB namespace
    pub surreal_ns: String,
    /// SurrealDB database
    pub surreal_db: String,
    /// SurrealDB username (for remote/root auth)
    pub surreal_user: Option<String>,
    /// SurrealDB password (for remote/root auth)
    pub surreal_pass: Option<String>,
    /// Raft configuration (for cluster)
    pub raft: Option<RaftConfig>,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            backend: StorageBackendType::Hybrid,
            data_dir: PathBuf::from("/var/lib/oxide-coordinator"),
            redis_url: Some("redis://127.0.0.1:6379".into()),
            surreal_url: None,
            surreal_ns: "oxide".into(),
            surreal_db: "mesh".into(),
            surreal_user: None,
            surreal_pass: None,
            raft: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageBackendType {
    Memory,
    Sled,
    Redis,
    SurrealMem,
    SurrealKv,
    SurrealWs,
    Hybrid,
    Raft,
}

/// Raft consensus configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaftConfig {
    /// Node ID
    pub node_id: u64,
    /// Cluster peers
    pub peers: Vec<RaftPeer>,
    /// Election timeout
    pub election_timeout: Duration,
    /// Heartbeat interval
    pub heartbeat_interval: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaftPeer {
    pub id: u64,
    pub address: SocketAddr,
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Enable authentication
    pub enabled: bool,
    /// JWT secret for token signing
    pub jwt_secret: String,
    /// JWT expiration
    pub jwt_expiration: Duration,
    /// OIDC configuration
    pub oidc: Option<OidcConfig>,
    /// Pre-shared enrollment tokens
    pub enrollment_tokens: Vec<String>,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            jwt_secret: "change-me-in-production".into(),
            jwt_expiration: Duration::from_secs(3600),
            oidc: None,
            enrollment_tokens: Vec::new(),
        }
    }
}

/// OIDC configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OidcConfig {
    /// Issuer URL
    pub issuer_url: String,
    /// Client ID
    pub client_id: String,
    /// Client secret
    pub client_secret: String,
    /// Redirect URL
    pub redirect_url: String,
    /// Scopes
    pub scopes: Vec<String>,
    /// Allowed domains
    pub allowed_domains: Vec<String>,
}

/// Cluster configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterConfig {
    /// Cluster name
    pub name: String,
    /// This node's ID
    pub node_id: String,
    /// Peer addresses
    pub peers: Vec<SocketAddr>,
    /// Gossip interval
    pub gossip_interval: Duration,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Enable rate limiting
    pub enabled: bool,
    /// Requests per second per IP
    pub requests_per_second: u32,
    /// Burst allowance
    pub burst: u32,
    /// MQTT publish rate limit
    pub mqtt_publish_per_second: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            requests_per_second: 100,
            burst: 200,
            mqtt_publish_per_second: 1000,
        }
    }
}

/// TLS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    /// Certificate file
    pub cert_file: PathBuf,
    /// Key file
    pub key_file: PathBuf,
    /// CA file (for mTLS)
    pub ca_file: Option<PathBuf>,
    /// Require client certificates
    pub require_client_cert: bool,
}