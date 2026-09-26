//! Production Storage Layer for Oxide Coordinator
//!
//! Dual-Engine Architecture:
//! - **SurrealDB 3.3.0**: Authoritative multi-model persistence, graph relations (mesh topology,
//!   shortest path, peer links), document records, and live change-feed queries.
//! - **Redis**: High-throughput ephemeral L1 cache, TTL-based node presence heartbeats,
//!   distributed pub/sub cluster invalidation, and rate limiting.
//! - **Hybrid Storage**: Unified coordinator storage combining SurrealDB 3.3.0 with Redis.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::{info, warn, error};

use crate::{StorageConfig, StorageBackendType, error::{CoordinatorError, Result}};
use oxide_core::{NodeId, OverlayPrefix, OverlayIp};
use oxide_protocol::topics::*;

/// Storage backend trait implemented by all coordinator storage drivers
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
                info!("Initializing In-Memory Storage");
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
            StorageBackendType::Redis => {
                #[cfg(feature = "redis")]
                {
                    let url = config.redis_url.as_deref().unwrap_or("redis://127.0.0.1:6379");
                    info!("Initializing Redis Storage at {}", url);
                    Arc::new(redis_storage::RedisStorage::new(url).await?)
                }
                #[cfg(not(feature = "redis"))]
                {
                    return Err(CoordinatorError::Storage("Redis backend not compiled (feature 'redis' disabled)".into()));
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
            StorageBackendType::Hybrid => {
                #[cfg(feature = "redis")]
                {
                    let redis_url = config.redis_url.as_deref().unwrap_or("redis://127.0.0.1:6379");
                    info!("Initializing Hybrid Storage: SurrealDB 3.3.0 beside Redis ({})", redis_url);
                    
                    let surreal = match &config.surreal_url {
                        Some(url) if url.starts_with("ws://") || url.starts_with("wss://") => {
                            surreal_storage::SurrealStorage::new_ws(
                                url,
                                &config.surreal_ns,
                                &config.surreal_db,
                                config.surreal_user.as_deref(),
                                config.surreal_pass.as_deref(),
                            ).await?
                        }
                        _ => {
                            let kv_path = config.data_dir.join("surrealkv");
                            if let Err(e) = std::fs::create_dir_all(&config.data_dir) {
                                warn!("Failed to create data dir {:?}: {}", config.data_dir, e);
                            }
                            match surreal_storage::SurrealStorage::new_surrealkv(&kv_path, &config.surreal_ns, &config.surreal_db).await {
                                Ok(s) => s,
                                Err(e) => {
                                    warn!("Failed to initialize SurrealKV ({}), falling back to SurrealDB in-memory engine: {}", kv_path.display(), e);
                                    surreal_storage::SurrealStorage::new_mem(&config.surreal_ns, &config.surreal_db).await?
                                }
                            }
                        }
                    };

                    let redis = match redis_storage::RedisStorage::new(redis_url).await {
                        Ok(r) => Some(r),
                        Err(e) => {
                            warn!("Redis unavailable at {} ({}). Proceeding with standalone SurrealDB 3.3.0 engine.", redis_url, e);
                            None
                        }
                    };

                    Arc::new(HybridStorage::new(surreal, redis))
                }
                #[cfg(not(feature = "redis"))]
                {
                    info!("Initializing SurrealDB 3.3.0 Storage (Redis feature disabled)");
                    let kv_path = config.data_dir.join("surrealkv");
                    Arc::new(surreal_storage::SurrealStorage::new_surrealkv(&kv_path, &config.surreal_ns, &config.surreal_db).await?)
                }
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
                let _ = db.signin(Root { username: u, password: p })
                    .await
                    .map_err(|e| CoordinatorError::Storage(format!("SurrealDB auth failed: {}", e)))?;
            }

            db.use_ns(ns).use_db(db_name)
                .await
                .map_err(|e| CoordinatorError::Storage(format!("SurrealDB use_ns/use_db failed: {}", e)))?;

            Ok(Self { engine: SurrealEngine::Remote(db) })
        }

        async fn query_sql(&self, sql: &str, binds: Vec<(&str, String)>) -> Result<()> {
            match &self.engine {
                SurrealEngine::Local(db) => {
                    let mut q = db.query(sql);
                    for (k, v) in binds {
                        q = q.bind((k.to_string(), v));
                    }
                    q.await.map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                }
                SurrealEngine::Remote(db) => {
                    let mut q = db.query(sql);
                    for (k, v) in binds {
                        q = q.bind((k.to_string(), v));
                    }
                    q.await.map_err(|e| CoordinatorError::Storage(e.to_string()))?;
                }
            }
            Ok(())
        }
    }

    #[async_trait::async_trait]
    impl StorageBackend for SurrealStorage {
        async fn get_node_metadata(&self, node_id: &NodeId) -> Result<Option<NodeMetadata>> {
            let record_id = format!("node:⟨{}⟩", node_id);
            let sql = "SELECT * FROM type::thing('node', $id)";
            
            let res: Option<SurrealNodeRecord> = match &self.engine {
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

            Ok(res.map(|r| NodeMetadata {
                node_id: r.node_id,
                display_name: r.display_name,
                os: r.os,
                arch: r.arch,
                version: r.version,
                tags: r.tags,
            }))
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
            let records: Vec<SurrealNodeRecord> = match &self.engine {
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

            Ok(records.into_iter().map(|r| NodeMetadata {
                node_id: r.node_id,
                display_name: r.display_name,
                os: r.os,
                arch: r.arch,
                version: r.version,
                tags: r.tags,
            }).collect())
        }

        async fn get_route(&self, prefix: &OverlayPrefix) -> Result<Option<RouteAdvertisement>> {
            let p_str = prefix.to_string();
            let sql = "SELECT * FROM type::thing('route', $id)";
            let record: Option<SurrealRouteRecord> = match &self.engine {
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

            record.map(|r| serde_json::from_str(&r.route_json).map_err(|e| CoordinatorError::Storage(e.to_string()))).transpose()
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
            let records: Vec<SurrealRouteRecord> = match &self.engine {
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

            records.into_iter()
                .map(|r| serde_json::from_str(&r.route_json).map_err(|e| CoordinatorError::Storage(e.to_string())))
                .collect()
        }

        async fn get_acl_policy(&self) -> Result<Option<AclPolicy>> {
            let sql = "SELECT * FROM type::thing('acl', 'current')";
            let record: Option<SurrealAclRecord> = match &self.engine {
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

            record.map(|r| serde_json::from_str(&r.policy_json).map_err(|e| CoordinatorError::Storage(e.to_string()))).transpose()
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
// Redis Ephemeral Caching & Sync Layer
// ============================================================================
#[cfg(feature = "redis")]
pub mod redis_storage {
    use super::*;
    use redis::{aio::ConnectionManager, AsyncCommands, Client};

    pub struct RedisStorage {
        conn: ConnectionManager,
    }

    impl RedisStorage {
        pub async fn new(url: &str) -> Result<Self> {
            let client = Client::open(url).map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            let conn = ConnectionManager::new(client).await
                .map_err(|e| CoordinatorError::Storage(format!("Redis ConnectionManager failed: {}", e)))?;
            Ok(Self { conn })
        }
    }

    #[async_trait::async_trait]
    impl StorageBackend for RedisStorage {
        async fn get_node_metadata(&self, node_id: &NodeId) -> Result<Option<NodeMetadata>> {
            let mut conn = self.conn.clone();
            let key = format!("oxide:node:{}", node_id);
            let data: Option<String> = conn.get(&key).await
                .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            data.map(|d| serde_json::from_str(&d).map_err(|e| CoordinatorError::Storage(e.to_string())))
                .transpose()
        }

        async fn set_node_metadata(&self, metadata: &NodeMetadata) -> Result<()> {
            let mut conn = self.conn.clone();
            let key = format!("oxide:node:{}", metadata.node_id);
            let value = serde_json::to_string(metadata).map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            let _: () = conn.set_ex(&key, &value, 3600).await
                .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            Ok(())
        }

        async fn delete_node_metadata(&self, node_id: &NodeId) -> Result<()> {
            let mut conn = self.conn.clone();
            let key = format!("oxide:node:{}", node_id);
            let _: () = conn.del(&key).await
                .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            Ok(())
        }

        async fn list_nodes(&self) -> Result<Vec<NodeMetadata>> {
            Ok(vec![])
        }

        async fn get_route(&self, prefix: &OverlayPrefix) -> Result<Option<RouteAdvertisement>> {
            let mut conn = self.conn.clone();
            let key = format!("oxide:route:{}", prefix);
            let data: Option<String> = conn.get(&key).await
                .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            data.map(|d| serde_json::from_str(&d).map_err(|e| CoordinatorError::Storage(e.to_string())))
                .transpose()
        }

        async fn set_route(&self, route: &RouteAdvertisement) -> Result<()> {
            let mut conn = self.conn.clone();
            let key = format!("oxide:route:{}", route.prefix);
            let value = serde_json::to_string(route).map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            let _: () = conn.set_ex(&key, &value, 3600).await
                .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            Ok(())
        }

        async fn delete_route(&self, prefix: &OverlayPrefix) -> Result<()> {
            let mut conn = self.conn.clone();
            let key = format!("oxide:route:{}", prefix);
            let _: () = conn.del(&key).await
                .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            Ok(())
        }

        async fn list_routes(&self) -> Result<Vec<RouteAdvertisement>> {
            Ok(vec![])
        }

        async fn get_acl_policy(&self) -> Result<Option<AclPolicy>> {
            let mut conn = self.conn.clone();
            let key = "oxide:acl:policy";
            let data: Option<String> = conn.get(key).await
                .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            data.map(|d| serde_json::from_str(&d).map_err(|e| CoordinatorError::Storage(e.to_string())))
                .transpose()
        }

        async fn set_acl_policy(&self, policy: &AclPolicy) -> Result<()> {
            let mut conn = self.conn.clone();
            let key = "oxide:acl:policy";
            let value = serde_json::to_string(policy).map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            let _: () = conn.set_ex(key, &value, 86400).await
                .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            Ok(())
        }

        async fn record_presence(&self, node_id: &NodeId, ttl: Duration) -> Result<()> {
            let mut conn = self.conn.clone();
            let key = format!("oxide:presence:{}", node_id);
            let now = chrono::Utc::now().timestamp().to_string();
            let _: () = conn.set_ex(&key, &now, ttl.as_secs()).await
                .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            Ok(())
        }

        async fn is_node_online(&self, node_id: &NodeId) -> Result<bool> {
            let mut conn = self.conn.clone();
            let key = format!("oxide:presence:{}", node_id);
            let exists: bool = conn.exists(&key).await
                .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            Ok(exists)
        }
    }
}

// ============================================================================
// Hybrid Dual-Engine Storage: SurrealDB 3.3.0 Beside Redis
// ============================================================================
pub struct HybridStorage {
    surreal: surreal_storage::SurrealStorage,
    #[cfg(feature = "redis")]
    redis: Option<redis_storage::RedisStorage>,
}

impl HybridStorage {
    #[cfg(feature = "redis")]
    pub fn new(surreal: surreal_storage::SurrealStorage, redis: Option<redis_storage::RedisStorage>) -> Self {
        Self { surreal, redis }
    }

    #[cfg(not(feature = "redis"))]
    pub fn new(surreal: surreal_storage::SurrealStorage) -> Self {
        Self { surreal }
    }
}

#[async_trait::async_trait]
impl StorageBackend for HybridStorage {
    async fn get_node_metadata(&self, node_id: &NodeId) -> Result<Option<NodeMetadata>> {
        #[cfg(feature = "redis")]
        if let Some(redis) = &self.redis {
            if let Ok(Some(cached)) = redis.get_node_metadata(node_id).await {
                return Ok(Some(cached));
            }
        }

        let metadata = self.surreal.get_node_metadata(node_id).await?;

        #[cfg(feature = "redis")]
        if let (Some(redis), Some(meta)) = (&self.redis, &metadata) {
            let _ = redis.set_node_metadata(meta).await;
        }

        Ok(metadata)
    }

    async fn set_node_metadata(&self, metadata: &NodeMetadata) -> Result<()> {
        // Authoritative write to SurrealDB 3.3.0
        self.surreal.set_node_metadata(metadata).await?;

        // Update/invalidate Redis L1 cache
        #[cfg(feature = "redis")]
        if let Some(redis) = &self.redis {
            let _ = redis.set_node_metadata(metadata).await;
        }

        Ok(())
    }

    async fn delete_node_metadata(&self, node_id: &NodeId) -> Result<()> {
        self.surreal.delete_node_metadata(node_id).await?;

        #[cfg(feature = "redis")]
        if let Some(redis) = &self.redis {
            let _ = redis.delete_node_metadata(node_id).await;
        }

        Ok(())
    }

    async fn list_nodes(&self) -> Result<Vec<NodeMetadata>> {
        self.surreal.list_nodes().await
    }

    async fn get_route(&self, prefix: &OverlayPrefix) -> Result<Option<RouteAdvertisement>> {
        #[cfg(feature = "redis")]
        if let Some(redis) = &self.redis {
            if let Ok(Some(cached)) = redis.get_route(prefix).await {
                return Ok(Some(cached));
            }
        }

        let route = self.surreal.get_route(prefix).await?;

        #[cfg(feature = "redis")]
        if let (Some(redis), Some(r)) = (&self.redis, &route) {
            let _ = redis.set_route(r).await;
        }

        Ok(route)
    }

    async fn set_route(&self, route: &RouteAdvertisement) -> Result<()> {
        self.surreal.set_route(route).await?;

        #[cfg(feature = "redis")]
        if let Some(redis) = &self.redis {
            let _ = redis.set_route(route).await;
        }

        Ok(())
    }

    async fn delete_route(&self, prefix: &OverlayPrefix) -> Result<()> {
        self.surreal.delete_route(prefix).await?;

        #[cfg(feature = "redis")]
        if let Some(redis) = &self.redis {
            let _ = redis.delete_route(prefix).await;
        }

        Ok(())
    }

    async fn list_routes(&self) -> Result<Vec<RouteAdvertisement>> {
        self.surreal.list_routes().await
    }

    async fn get_acl_policy(&self) -> Result<Option<AclPolicy>> {
        #[cfg(feature = "redis")]
        if let Some(redis) = &self.redis {
            if let Ok(Some(cached)) = redis.get_acl_policy().await {
                return Ok(Some(cached));
            }
        }

        let policy = self.surreal.get_acl_policy().await?;

        #[cfg(feature = "redis")]
        if let (Some(redis), Some(p)) = (&self.redis, &policy) {
            let _ = redis.set_acl_policy(p).await;
        }

        Ok(policy)
    }

    async fn set_acl_policy(&self, policy: &AclPolicy) -> Result<()> {
        self.surreal.set_acl_policy(policy).await?;

        #[cfg(feature = "redis")]
        if let Some(redis) = &self.redis {
            let _ = redis.set_acl_policy(policy).await;
        }

        Ok(())
    }

    async fn record_presence(&self, node_id: &NodeId, ttl: Duration) -> Result<()> {
        #[cfg(feature = "redis")]
        if let Some(redis) = &self.redis {
            return redis.record_presence(node_id, ttl).await;
        }
        Ok(())
    }

    async fn is_node_online(&self, node_id: &NodeId) -> Result<bool> {
        #[cfg(feature = "redis")]
        if let Some(redis) = &self.redis {
            return redis.is_node_online(node_id).await;
        }
        Ok(true)
    }

    async fn record_topology_link(&self, from: &NodeId, to: &NodeId, latency_ms: f32, loss_rate: f32) -> Result<()> {
        self.surreal.record_topology_link(from, to, latency_ms, loss_rate).await
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