//! Production Storage Layer for Oxide Coordinator using SurrealDB 3.3.0
//!
//! Multi-Model & Graph Architecture:
//! - **SurrealDB 3.3.0**: Authoritative persistence, graph relations (mesh topology,
//!   shortest path, peer links), document records, and live change-feed queries.
//! - In-Memory (`Mem`), Embedded Disk (`SurrealKv`), or Distributed Remote (`Ws`).

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::info;

use crate::{StorageConfig, StorageBackendType, error::{CoordinatorError, Result}};
use oxide_core::{NodeId, OverlayPrefix};
use oxide_protocol::topics::*;

/// Storage backend trait implemented by coordinator storage drivers
#[async_trait::async_trait]
pub trait StorageBackend: Send + Sync {
    async fn get_node_metadata(&self, node_id: &NodeId) -> Result<Option<NodeMetadata>>;
    async fn set_node_metadata(&self, metadata: &NodeMetadata) -> Result<()>;
    async fn delete_node_metadata(&self, node_id: &NodeId) -> Result<()>;
    async fn list_nodes(&self) -> Result<Vec<NodeMetadata>>;

    async fn get_route(&self, prefix: &OverlayPrefix) -> Result<Option<RouteAdvertisement>>;
    async fn set_route(&self, route: &RouteAdvertisement) -> Result<()>;
    async fn delete_route(&self, prefix: &OverlayPrefix) -> Result<()>;
    async fn list_routes(&self) -> Result<Vec<RouteAdvertisement>>;

    async fn get_acl_policy(&self) -> Result<Option<AclPolicy>>;
    async fn set_acl_policy(&self, policy: &AclPolicy) -> Result<()>;

    // Extended capabilities (presence & graph)
    async fn record_presence(&self, _node_id: &NodeId, _ttl: Duration) -> Result<()> {
        Ok(())
    }

    async fn is_node_online(&self, _node_id: &NodeId) -> Result<bool> {
        Ok(true)
    }

    async fn record_topology_link(&self, _from: &NodeId, _to: &NodeId, _latency_ms: f32, _loss_rate: f32) -> Result<()> {
        Ok(())
    }
}

/// Main storage interface holding the configured backend
pub struct Storage {
    backend: Arc<dyn StorageBackend>,
}

impl Storage {
    /// Asynchronously initialize storage according to configuration
    pub async fn new_async(config: &StorageConfig) -> Result<Self> {
        let backend: Arc<dyn StorageBackend> = match config.backend {
            StorageBackendType::Memory => {
                info!("Initializing In-Memory Fallback Storage");
                Arc::new(MemoryStorage::new())
            }
            StorageBackendType::Sled => {
                #[cfg(feature = "sled")]
                {
                    info!("Initializing Sled Storage at {:?}", config.data_dir);
                    Arc::new(sled_storage::SledStorage::new(&config.data_dir)?)
                }
                #[cfg(not(feature = "sled"))]
                {
                    return Err(CoordinatorError::Storage("Sled backend not compiled (feature 'sled' disabled)".into()));
                }
            }
            StorageBackendType::SurrealMem => {
                info!("Initializing SurrealDB 3.3.0 In-Memory Engine (ns={}, db={})", config.surreal_ns, config.surreal_db);
                Arc::new(surreal_storage::SurrealStorage::new_mem(&config.surreal_ns, &config.surreal_db).await?)
            }
            StorageBackendType::SurrealKv => {
                let kv_path = config.data_dir.join("surrealkv");
                info!("Initializing SurrealDB 3.3.0 SurrealKV Disk Engine at {:?} (ns={}, db={})", kv_path, config.surreal_ns, config.surreal_db);
                Arc::new(surreal_storage::SurrealStorage::new_surrealkv(&kv_path, &config.surreal_ns, &config.surreal_db).await?)
            }
            StorageBackendType::SurrealWs => {
                let url = config.surreal_url.as_deref().unwrap_or("ws://127.0.0.1:8000");
                info!("Connecting to remote SurrealDB 3.3.0 cluster at {} (ns={}, db={})", url, config.surreal_ns, config.surreal_db);
                Arc::new(surreal_storage::SurrealStorage::new_ws(
                    url,
                    &config.surreal_ns,
                    &config.surreal_db,
                    config.surreal_user.as_deref(),
                    config.surreal_pass.as_deref(),
                ).await?)
            }
            StorageBackendType::Raft => {
                return Err(CoordinatorError::Storage("Raft consensus backend not yet implemented".into()));
            }
        };

        Ok(Self { backend })
    }

    /// Synchronous initialization wrapper (defaults to in-memory or blocks on async runtime)
    pub fn new(config: &StorageConfig) -> Result<Self> {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(Self::new_async(config))
        })
    }

    pub async fn get_node_metadata(&self, node_id: &NodeId) -> Result<Option<NodeMetadata>> {
        self.backend.get_node_metadata(node_id).await
    }

    pub async fn set_node_metadata(&self, metadata: &NodeMetadata) -> Result<()> {
        self.backend.set_node_metadata(metadata).await
    }

    pub async fn delete_node_metadata(&self, node_id: &NodeId) -> Result<()> {
        self.backend.delete_node_metadata(node_id).await
    }

    pub async fn list_nodes(&self) -> Result<Vec<NodeMetadata>> {
        self.backend.list_nodes().await
    }

    pub async fn get_route(&self, prefix: &OverlayPrefix) -> Result<Option<RouteAdvertisement>> {
        self.backend.get_route(prefix).await
    }

    pub async fn set_route(&self, route: &RouteAdvertisement) -> Result<()> {
        self.backend.set_route(route).await
    }

    pub async fn delete_route(&self, prefix: &OverlayPrefix) -> Result<()> {
        self.backend.delete_route(prefix).await
    }

    pub async fn list_routes(&self) -> Result<Vec<RouteAdvertisement>> {
        self.backend.list_routes().await
    }

    pub async fn get_acl_policy(&self) -> Result<Option<AclPolicy>> {
        self.backend.get_acl_policy().await
    }

    pub async fn set_acl_policy(&self, policy: &AclPolicy) -> Result<()> {
        self.backend.set_acl_policy(policy).await
    }

    pub async fn record_presence(&self, node_id: &NodeId, ttl: Duration) -> Result<()> {
        self.backend.record_presence(node_id, ttl).await
    }

    pub async fn is_node_online(&self, node_id: &NodeId) -> Result<bool> {
        self.backend.is_node_online(node_id).await
    }

    pub async fn record_topology_link(&self, from: &NodeId, to: &NodeId, latency_ms: f32, loss_rate: f32) -> Result<()> {
        self.backend.record_topology_link(from, to, latency_ms, loss_rate).await
    }
}

// ============================================================================
// SurrealDB 3.3.0 Engine Implementation
// ============================================================================
pub mod surreal_storage {
    use super::*;
    use surrealdb::Surreal;
    use surrealdb::engine::local::{Db, Mem, SurrealKv};
    use surrealdb::engine::remote::ws::{Client as WsClient, Ws};
    use surrealdb::opt::auth::Root;

    enum SurrealEngine {
        Local(Surreal<Db>),
        Remote(Surreal<WsClient>),
    }

    pub struct SurrealStorage {
        engine: SurrealEngine,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct SurrealNodeRecord {
        pub id: String,
        pub node_id: NodeId,
        pub display_name: Option<String>,
        pub os: String,
        pub arch: String,
        pub version: String,
        pub tags: Vec<String>,
        pub updated_at: i64,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct SurrealRouteRecord {
        pub id: String,
        pub prefix_str: String,
        pub route_json: String,
        pub updated_at: i64,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct SurrealAclRecord {
        pub id: String,
        pub policy_json: String,
        pub version: u64,
        pub updated_at: i64,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct SurrealPresenceRecord {
        pub id: String,
        pub node_id: NodeId,
        pub expires_at: i64,
    }

    impl SurrealStorage {
        pub async fn new_mem(ns: &str, db_name: &str) -> Result<Self> {
            let db = Surreal::new::<Mem>(())
                .await
                .map_err(|e| CoordinatorError::Storage(format!("SurrealDB Mem init failed: {}", e)))?;
            
            db.use_ns(ns).use_db(db_name)
                .await
                .map_err(|e| CoordinatorError::Storage(format!("SurrealDB use_ns/use_db failed: {}", e)))?;

            Ok(Self { engine: SurrealEngine::Local(db) })
        }

        pub async fn new_surrealkv(path: &PathBuf, ns: &str, db_name: &str) -> Result<Self> {
            let path_str = path.to_string_lossy().to_string();
            let db = Surreal::new::<SurrealKv>(path_str)
                .await
                .map_err(|e| CoordinatorError::Storage(format!("SurrealDB SurrealKV init failed: {}", e)))?;

            db.use_ns(ns).use_db(db_name)
                .await
                .map_err(|e| CoordinatorError::Storage(format!("SurrealDB use_ns/use_db failed: {}", e)))?;

            Ok(Self { engine: SurrealEngine::Local(db) })
        }

        pub async fn new_ws(url: &str, ns: &str, db_name: &str, user: Option<&str>, pass: Option<&str>) -> Result<Self> {
            let db = Surreal::new::<Ws>(url)
                .await
                .map_err(|e| CoordinatorError::Storage(format!("SurrealDB remote WS connect failed: {}", e)))?;

            if let (Some(u), Some(p)) = (user, pass) {
                let _ = db.signin(Root { username: u.to_string(), password: p.to_string() })
                    .await
                    .map_err(|e| CoordinatorError::Storage(format!("SurrealDB auth failed: {}", e)))?;
            }

            db.use_ns(ns).use_db(db_name)
                .await
                .map_err(|e| CoordinatorError::Storage(format!("SurrealDB use_ns/use_db failed: {}", e)))?;

            Ok(Self { engine: SurrealEngine::Remote(db) })
        }
    }

    #[async_trait::async_trait]
    impl StorageBackend for SurrealStorage {
        async fn get_node_metadata(&self, node_id: &NodeId) -> Result<Option<NodeMetadata>> {
            let sql = "SELECT * FROM type::thing('node', $id)";
            
            let val: Option<surrealdb::types::Value> = match &self.engine {
                SurrealEngine::Local(db) => {
                    let mut resp = db.query(sql).bind(("id", node_id.to_string())).await
                        .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                    resp.take(0).map_err(|e| CoordinatorError::Storage(e.to_string()))?
                }
                SurrealEngine::Remote(db) => {
                    let mut resp = db.query(sql).bind(("id", node_id.to_string())).await
                        .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                    resp.take(0).map_err(|e| CoordinatorError::Storage(e.to_string()))?
                }
            };

            if let Some(v) = val {
                let json_str = serde_json::to_string(&v).map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                let r: SurrealNodeRecord = serde_json::from_str(&json_str).map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                Ok(Some(NodeMetadata {
                    node_id: r.node_id,
                    display_name: r.display_name,
                    os: r.os,
                    arch: r.arch,
                    version: r.version,
                    tags: r.tags,
                }))
            } else {
                Ok(None)
            }
        }

        async fn set_node_metadata(&self, metadata: &NodeMetadata) -> Result<()> {
            let record = SurrealNodeRecord {
                id: format!("node:⟨{}⟩", metadata.node_id),
                node_id: metadata.node_id,
                display_name: metadata.display_name.clone(),
                os: metadata.os.clone(),
                arch: metadata.arch.clone(),
                version: metadata.version.clone(),
                tags: metadata.tags.clone(),
                updated_at: chrono::Utc::now().timestamp(),
            };

            let sql = "UPSERT type::thing('node', $id) CONTENT $content";
            let val = serde_json::to_value(&record).map_err(|e| CoordinatorError::Storage(e.to_string()))?;

            match &self.engine {
                SurrealEngine::Local(db) => {
                    db.query(sql)
                        .bind(("id", metadata.node_id.to_string()))
                        .bind(("content", val))
                        .await
                        .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                }
                SurrealEngine::Remote(db) => {
                    db.query(sql)
                        .bind(("id", metadata.node_id.to_string()))
                        .bind(("content", val))
                        .await
                        .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                }
            }
            Ok(())
        }

        async fn delete_node_metadata(&self, node_id: &NodeId) -> Result<()> {
            let sql = "DELETE type::thing('node', $id)";
            match &self.engine {
                SurrealEngine::Local(db) => {
                    db.query(sql).bind(("id", node_id.to_string())).await
                        .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                }
                SurrealEngine::Remote(db) => {
                    db.query(sql).bind(("id", node_id.to_string())).await
                        .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                }
            }
            Ok(())
        }

        async fn list_nodes(&self) -> Result<Vec<NodeMetadata>> {
            let sql = "SELECT * FROM node";
            let vals: Vec<surrealdb::types::Value> = match &self.engine {
                SurrealEngine::Local(db) => {
                    let mut resp = db.query(sql).await
                        .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                    resp.take(0).map_err(|e| CoordinatorError::Storage(e.to_string()))?
                }
                SurrealEngine::Remote(db) => {
                    let mut resp = db.query(sql).await
                        .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                    resp.take(0).map_err(|e| CoordinatorError::Storage(e.to_string()))?
                }
            };

            let mut nodes = Vec::new();
            for v in vals {
                let json_str = serde_json::to_string(&v).map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                if let Ok(r) = serde_json::from_str::<SurrealNodeRecord>(&json_str) {
                    nodes.push(NodeMetadata {
                        node_id: r.node_id,
                        display_name: r.display_name,
                        os: r.os,
                        arch: r.arch,
                        version: r.version,
                        tags: r.tags,
                    });
                }
            }
            Ok(nodes)
        }

        async fn get_route(&self, prefix: &OverlayPrefix) -> Result<Option<RouteAdvertisement>> {
            let p_str = prefix.to_string();
            let sql = "SELECT * FROM type::thing('route', $id)";
            let val: Option<surrealdb::types::Value> = match &self.engine {
                SurrealEngine::Local(db) => {
                    let mut resp = db.query(sql).bind(("id", p_str)).await
                        .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                    resp.take(0).map_err(|e| CoordinatorError::Storage(e.to_string()))?
                }
                SurrealEngine::Remote(db) => {
                    let mut resp = db.query(sql).bind(("id", p_str)).await
                        .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                    resp.take(0).map_err(|e| CoordinatorError::Storage(e.to_string()))?
                }
            };

            if let Some(v) = val {
                let json_str = serde_json::to_string(&v).map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                let r: SurrealRouteRecord = serde_json::from_str(&json_str).map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                let route: RouteAdvertisement = serde_json::from_str(&r.route_json).map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                Ok(Some(route))
            } else {
                Ok(None)
            }
        }

        async fn set_route(&self, route: &RouteAdvertisement) -> Result<()> {
            let p_str = route.prefix.to_string();
            let route_json = serde_json::to_string(route).map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            let record = SurrealRouteRecord {
                id: format!("route:⟨{}⟩", p_str),
                prefix_str: p_str.clone(),
                route_json,
                updated_at: chrono::Utc::now().timestamp(),
            };

            let sql = "UPSERT type::thing('route', $id) CONTENT $content";
            let val = serde_json::to_value(&record).map_err(|e| CoordinatorError::Storage(e.to_string()))?;

            match &self.engine {
                SurrealEngine::Local(db) => {
                    db.query(sql).bind(("id", p_str)).bind(("content", val)).await
                        .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                }
                SurrealEngine::Remote(db) => {
                    db.query(sql).bind(("id", p_str)).bind(("content", val)).await
                        .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                }
            }
            Ok(())
        }

        async fn delete_route(&self, prefix: &OverlayPrefix) -> Result<()> {
            let sql = "DELETE type::thing('route', $id)";
            let p_str = prefix.to_string();
            match &self.engine {
                SurrealEngine::Local(db) => {
                    db.query(sql).bind(("id", p_str)).await
                        .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                }
                SurrealEngine::Remote(db) => {
                    db.query(sql).bind(("id", p_str)).await
                        .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                }
            }
            Ok(())
        }

        async fn list_routes(&self) -> Result<Vec<RouteAdvertisement>> {
            let sql = "SELECT * FROM route";
            let vals: Vec<surrealdb::types::Value> = match &self.engine {
                SurrealEngine::Local(db) => {
                    let mut resp = db.query(sql).await
                        .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                    resp.take(0).map_err(|e| CoordinatorError::Storage(e.to_string()))?
                }
                SurrealEngine::Remote(db) => {
                    let mut resp = db.query(sql).await
                        .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                    resp.take(0).map_err(|e| CoordinatorError::Storage(e.to_string()))?
                }
            };

            let mut routes = Vec::new();
            for v in vals {
                let json_str = serde_json::to_string(&v).map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                if let Ok(r) = serde_json::from_str::<SurrealRouteRecord>(&json_str) {
                    if let Ok(route) = serde_json::from_str::<RouteAdvertisement>(&r.route_json) {
                        routes.push(route);
                    }
                }
            }
            Ok(routes)
        }

        async fn get_acl_policy(&self) -> Result<Option<AclPolicy>> {
            let sql = "SELECT * FROM type::thing('acl', 'current')";
            let val: Option<surrealdb::types::Value> = match &self.engine {
                SurrealEngine::Local(db) => {
                    let mut resp = db.query(sql).await
                        .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                    resp.take(0).map_err(|e| CoordinatorError::Storage(e.to_string()))?
                }
                SurrealEngine::Remote(db) => {
                    let mut resp = db.query(sql).await
                        .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                    resp.take(0).map_err(|e| CoordinatorError::Storage(e.to_string()))?
                }
            };

            if let Some(v) = val {
                let json_str = serde_json::to_string(&v).map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                let r: SurrealAclRecord = serde_json::from_str(&json_str).map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                let policy: AclPolicy = serde_json::from_str(&r.policy_json).map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                Ok(Some(policy))
            } else {
                Ok(None)
            }
        }

        async fn set_acl_policy(&self, policy: &AclPolicy) -> Result<()> {
            let policy_json = serde_json::to_string(policy).map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            let record = SurrealAclRecord {
                id: "acl:current".into(),
                policy_json,
                version: policy.version,
                updated_at: chrono::Utc::now().timestamp(),
            };

            let sql = "UPSERT type::thing('acl', 'current') CONTENT $content";
            let val = serde_json::to_value(&record).map_err(|e| CoordinatorError::Storage(e.to_string()))?;

            match &self.engine {
                SurrealEngine::Local(db) => {
                    db.query(sql).bind(("content", val)).await
                        .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                }
                SurrealEngine::Remote(db) => {
                    db.query(sql).bind(("content", val)).await
                        .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                }
            }
            Ok(())
        }

        async fn record_presence(&self, node_id: &NodeId, ttl: Duration) -> Result<()> {
            let expires_at = chrono::Utc::now().timestamp() + ttl.as_secs() as i64;
            let record = SurrealPresenceRecord {
                id: format!("presence:⟨{}⟩", node_id),
                node_id: *node_id,
                expires_at,
            };
            let sql = "UPSERT type::thing('presence', $id) CONTENT $content";
            let val = serde_json::to_value(&record).map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            match &self.engine {
                SurrealEngine::Local(db) => {
                    db.query(sql).bind(("id", node_id.to_string())).bind(("content", val)).await
                        .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                }
                SurrealEngine::Remote(db) => {
                    db.query(sql).bind(("id", node_id.to_string())).bind(("content", val)).await
                        .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                }
            }
            Ok(())
        }

        async fn is_node_online(&self, node_id: &NodeId) -> Result<bool> {
            let now = chrono::Utc::now().timestamp();
            let sql = "SELECT * FROM type::thing('presence', $id) WHERE expires_at > $now";
            let val: Option<surrealdb::types::Value> = match &self.engine {
                SurrealEngine::Local(db) => {
                    let mut resp = db.query(sql).bind(("id", node_id.to_string())).bind(("now", now)).await
                        .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                    resp.take(0).map_err(|e| CoordinatorError::Storage(e.to_string()))?
                }
                SurrealEngine::Remote(db) => {
                    let mut resp = db.query(sql).bind(("id", node_id.to_string())).bind(("now", now)).await
                        .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                    resp.take(0).map_err(|e| CoordinatorError::Storage(e.to_string()))?
                }
            };
            Ok(val.is_some())
        }

        async fn record_topology_link(&self, from: &NodeId, to: &NodeId, latency_ms: f32, loss_rate: f32) -> Result<()> {
            // SurrealQL Graph Relation: RELATE node:from->connected_to->node:to
            let sql = "RELATE type::thing('node', $from)->connected_to->type::thing('node', $to) SET latency_ms = $latency, loss_rate = $loss, updated_at = time::now()";
            match &self.engine {
                SurrealEngine::Local(db) => {
                    db.query(sql)
                        .bind(("from", from.to_string()))
                        .bind(("to", to.to_string()))
                        .bind(("latency", latency_ms))
                        .bind(("loss", loss_rate))
                        .await
                        .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                }
                SurrealEngine::Remote(db) => {
                    db.query(sql)
                        .bind(("from", from.to_string()))
                        .bind(("to", to.to_string()))
                        .bind(("latency", latency_ms))
                        .bind(("loss", loss_rate))
                        .await
                        .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                }
            }
            Ok(())
        }
    }
}

// ============================================================================
// In-Memory Storage (Fallback / Mock)
// ============================================================================
struct MemoryStorage {
    nodes: Arc<RwLock<HashMap<NodeId, NodeMetadata>>>,
    routes: Arc<RwLock<HashMap<OverlayPrefix, RouteAdvertisement>>>,
    acl_policy: Arc<RwLock<Option<AclPolicy>>>,
}

impl MemoryStorage {
    fn new() -> Self {
        Self {
            nodes: Arc::new(RwLock::new(HashMap::new())),
            routes: Arc::new(RwLock::new(HashMap::new())),
            acl_policy: Arc::new(RwLock::new(None)),
        }
    }
}

#[async_trait::async_trait]
impl StorageBackend for MemoryStorage {
    async fn get_node_metadata(&self, node_id: &NodeId) -> Result<Option<NodeMetadata>> {
        Ok(self.nodes.read().await.get(node_id).cloned())
    }

    async fn set_node_metadata(&self, metadata: &NodeMetadata) -> Result<()> {
        self.nodes.write().await.insert(metadata.node_id, metadata.clone());
        Ok(())
    }

    async fn delete_node_metadata(&self, node_id: &NodeId) -> Result<()> {
        self.nodes.write().await.remove(node_id);
        Ok(())
    }

    async fn list_nodes(&self) -> Result<Vec<NodeMetadata>> {
        Ok(self.nodes.read().await.values().cloned().collect())
    }

    async fn get_route(&self, prefix: &OverlayPrefix) -> Result<Option<RouteAdvertisement>> {
        Ok(self.routes.read().await.get(prefix).cloned())
    }

    async fn set_route(&self, route: &RouteAdvertisement) -> Result<()> {
        self.routes.write().await.insert(route.prefix, route.clone());
        Ok(())
    }

    async fn delete_route(&self, prefix: &OverlayPrefix) -> Result<()> {
        self.routes.write().await.remove(prefix);
        Ok(())
    }

    async fn list_routes(&self) -> Result<Vec<RouteAdvertisement>> {
        Ok(self.routes.read().await.values().cloned().collect())
    }

    async fn get_acl_policy(&self) -> Result<Option<AclPolicy>> {
        Ok(self.acl_policy.read().await.clone())
    }

    async fn set_acl_policy(&self, policy: &AclPolicy) -> Result<()> {
        *self.acl_policy.write().await = Some(policy.clone());
        Ok(())
    }
}

// ============================================================================
// Sled Embedded Storage
// ============================================================================
#[cfg(feature = "sled")]
mod sled_storage {
    use super::*;
    use sled::Db;

    pub struct SledStorage {
        db: Db,
        nodes_tree: sled::Tree,
        routes_tree: sled::Tree,
        acl_tree: sled::Tree,
    }

    impl SledStorage {
        pub fn new(data_dir: &PathBuf) -> Result<Self> {
            let db = sled::open(data_dir).map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            let nodes_tree = db.open_tree("nodes").map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            let routes_tree = db.open_tree("routes").map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            let acl_tree = db.open_tree("acl").map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            Ok(Self { db, nodes_tree, routes_tree, acl_tree })
        }
    }

    #[async_trait::async_trait]
    impl StorageBackend for SledStorage {
        async fn get_node_metadata(&self, node_id: &NodeId) -> Result<Option<NodeMetadata>> {
            let key = node_id.as_bytes();
            self.nodes_tree.get(key)
                .map_err(|e| CoordinatorError::Storage(e.to_string()))?
                .map(|v| postcard::from_bytes(&v).map_err(|e| CoordinatorError::Storage(e.to_string())))
                .transpose()
        }

        async fn set_node_metadata(&self, metadata: &NodeMetadata) -> Result<()> {
            let key = metadata.node_id.as_bytes();
            let value = postcard::to_stdvec(metadata).map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            self.nodes_tree.insert(key, value).map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            Ok(())
        }

        async fn delete_node_metadata(&self, node_id: &NodeId) -> Result<()> {
            self.nodes_tree.remove(node_id.as_bytes()).map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            Ok(())
        }

        async fn list_nodes(&self) -> Result<Vec<NodeMetadata>> {
            self.nodes_tree.iter()
                .filter_map(|r| r.ok())
                .filter_map(|(_, v)| postcard::from_bytes(&v).ok())
                .collect()
        }

        async fn get_route(&self, prefix: &OverlayPrefix) -> Result<Option<RouteAdvertisement>> {
            let key = postcard::to_stdvec(prefix).map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            self.routes_tree.get(key)
                .map_err(|e| CoordinatorError::Storage(e.to_string()))?
                .map(|v| postcard::from_bytes(&v).map_err(|e| CoordinatorError::Storage(e.to_string())))
                .transpose()
        }

        async fn set_route(&self, route: &RouteAdvertisement) -> Result<()> {
            let key = postcard::to_stdvec(&route.prefix).map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            let value = postcard::to_stdvec(route).map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            self.routes_tree.insert(key, value).map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            Ok(())
        }

        async fn delete_route(&self, prefix: &OverlayPrefix) -> Result<()> {
            let key = postcard::to_stdvec(prefix).map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            self.routes_tree.remove(key).map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            Ok(())
        }

        async fn list_routes(&self) -> Result<Vec<RouteAdvertisement>> {
            self.routes_tree.iter()
                .filter_map(|r| r.ok())
                .filter_map(|(_, v)| postcard::from_bytes(&v).ok())
                .collect()
        }

        async fn get_acl_policy(&self) -> Result<Option<AclPolicy>> {
            self.acl_tree.get("policy")
                .map_err(|e| CoordinatorError::Storage(e.to_string()))?
                .map(|v| postcard::from_bytes(&v).map_err(|e| CoordinatorError::Storage(e.to_string())))
                .transpose()
        }

        async fn set_acl_policy(&self, policy: &AclPolicy) -> Result<()> {
            let value = postcard::to_stdvec(policy).map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            self.acl_tree.insert("policy", value).map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_surreal_storage_nodes() {
        let storage = surreal_storage::SurrealStorage::new_mem("test_ns", "test_db")
            .await
            .expect("SurrealDB Mem init");

        let node_id = NodeId::new();
        let metadata = NodeMetadata {
            node_id,
            display_name: Some("test-edge-1".into()),
            os: "linux".into(),
            arch: "x86_64".into(),
            version: "0.1.0".into(),
            tags: vec!["gateway".into(), "edge".into()],
        };

        // Insert
        storage.set_node_metadata(&metadata).await.expect("set_node_metadata");

        // Retrieve
        let retrieved = storage.get_node_metadata(&node_id).await.expect("get_node_metadata");
        assert!(retrieved.is_some());
        let r = retrieved.unwrap();
        assert_eq!(r.node_id, node_id);
        assert_eq!(r.display_name.as_deref(), Some("test-edge-1"));
        assert_eq!(r.tags, vec!["gateway".to_string(), "edge".to_string()]);

        // List
        let list = storage.list_nodes().await.expect("list_nodes");
        assert_eq!(list.len(), 1);

        // Delete
        storage.delete_node_metadata(&node_id).await.expect("delete_node_metadata");
        let retrieved_after = storage.get_node_metadata(&node_id).await.expect("get after delete");
        assert!(retrieved_after.is_none());
    }

    #[tokio::test]
    async fn test_surreal_storage_routes_and_topology() {
        let storage = surreal_storage::SurrealStorage::new_mem("test_ns", "test_db")
            .await
            .expect("SurrealDB Mem init");

        let prefix = "100.64.10.0/24".parse().unwrap();
        let node_id = NodeId::new();
        let peer_id = NodeId::new();

        let route = RouteAdvertisement {
            prefix,
            node_id,
            metric: 10,
            direct_next_hop: None,
            active: true,
            last_advertised: chrono::Utc::now().timestamp(),
        };

        // Set route
        storage.set_route(&route).await.expect("set_route");

        // Get route
        let r = storage.get_route(&prefix).await.expect("get_route");
        assert!(r.is_some());
        assert_eq!(r.unwrap().metric, 10);

        // List routes
        let routes = storage.list_routes().await.expect("list_routes");
        assert_eq!(routes.len(), 1);

        // Record graph topology link
        storage.record_topology_link(&node_id, &peer_id, 14.5, 0.001)
            .await
            .expect("record_topology_link");

        // Presence check
        storage.record_presence(&node_id, Duration::from_secs(60)).await.expect("record_presence");
        let online = storage.is_node_online(&node_id).await.expect("is_node_online");
        assert!(online);

        // Delete route
        storage.delete_route(&prefix).await.expect("delete_route");
        assert!(storage.get_route(&prefix).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_memory_storage_fallback() {
        let storage = MemoryStorage::new();
        let node_id = NodeId::new();
        let metadata = NodeMetadata {
            node_id,
            display_name: Some("mem-node".into()),
            os: "macos".into(),
            arch: "aarch64".into(),
            version: "0.1.0".into(),
            tags: vec![],
        };

        storage.set_node_metadata(&metadata).await.unwrap();
        let res = storage.get_node_metadata(&node_id).await.unwrap();
        assert!(res.is_some());
        assert_eq!(res.unwrap().display_name.as_deref(), Some("mem-node"));
    }
}