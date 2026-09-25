//! Storage layer for coordinator

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use crate::{StorageConfig, StorageBackend, error::{CoordinatorError, Result}};
use oxide_core::{NodeId, MeshName, OverlayPrefix, OverlayIp, Endpoint, NodeCapabilities};
use oxide_crypto::keys::{DeviceIdentityPublicKey, SessionPublicKey, KeyFingerprint};
use oxide_protocol::topics::*;

/// Storage backend trait
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
}

/// Main storage interface
pub struct Storage {
    backend: Box<dyn StorageBackend>,
}

impl Storage {
    pub fn new(config: &StorageConfig) -> Result<Self> {
        let backend: Box<dyn StorageBackend> = match config.backend {
            StorageBackend::Memory => Box::new(MemoryStorage::new()),
            StorageBackend::Sled => {
                #[cfg(feature = "sled")]
                {
                    Box::new(SledStorage::new(&config.data_dir)?)
                }
                #[cfg(not(feature = "sled"))]
                {
                    return Err(CoordinatorError::Storage("Sled backend not compiled".into()));
                }
            }
            StorageBackend::Redis => {
                #[cfg(feature = "redis")]
                {
                    Box::new(RedisStorage::new(config.redis_url.as_deref().unwrap_or("redis://127.0.0.1:6379"))?)
                }
                #[cfg(not(feature = "redis"))]
                {
                    return Err(CoordinatorError::Storage("Redis backend not compiled".into()));
                }
            }
            StorageBackend::Raft => {
                return Err(CoordinatorError::Storage("Raft backend not yet implemented".into()));
            }
        };

        Ok(Self { backend })
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
}

/// In-memory storage implementation
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

/// Sled storage (optional)
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
            // Serialize prefix as key
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

/// Redis storage (optional)
#[cfg(feature = "redis")]
mod redis_storage {
    use super::*;
    use redis::{Client, AsyncCommands};

    pub struct RedisStorage {
        client: Client,
    }

    impl RedisStorage {
        pub fn new(url: &str) -> Result<Self> {
            let client = Client::open(url).map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            Ok(Self { client })
        }
    }

    #[async_trait::async_trait]
    impl StorageBackend for RedisStorage {
        async fn get_node_metadata(&self, node_id: &NodeId) -> Result<Option<NodeMetadata>> {
            let mut conn = self.client.get_multiplexed_async_connection().await
                .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            let key = format!("node:{}", node_id);
            let data: Option<String> = conn.get(&key).await
                .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            data.map(|d| serde_json::from_str(&d).map_err(|e| CoordinatorError::Storage(e.to_string())))
                .transpose()
        }

        async fn set_node_metadata(&self, metadata: &NodeMetadata) -> Result<()> {
            let mut conn = self.client.get_multiplexed_async_connection().await
                .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            let key = format!("node:{}", metadata.node_id);
            let value = serde_json::to_string(metadata).map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            conn.set(&key, &value).await.map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            Ok(())
        }

        async fn delete_node_metadata(&self, node_id: &NodeId) -> Result<()> {
            let mut conn = self.client.get_multiplexed_async_connection().await
                .map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            let key = format!("node:{}", node_id);
            conn.del(&key).await.map_err(|e| CoordinatorError::Storage(e.to_string()))?;
            Ok(())
        }

        async fn list_nodes(&self) -> Result<Vec<NodeMetadata>> {
            // Would use SCAN
            Ok(vec![])
        }

        async fn get_route(&self, prefix: &OverlayPrefix) -> Result<Option<RouteAdvertisement>> {
            Ok(None)
        }

        async fn set_route(&self, route: &RouteAdvertisement) -> Result<()> {
            Ok(())
        }

        async fn delete_route(&self, prefix: &OverlayPrefix) -> Result<()> {
            Ok(())
        }

        async fn list_routes(&self) -> Result<Vec<RouteAdvertisement>> {
            Ok(vec![])
        }

        async fn get_acl_policy(&self) -> Result<Option<AclPolicy>> {
            Ok(None)
        }

        async fn set_acl_policy(&self, policy: &AclPolicy) -> Result<()> {
            Ok(())
        }
    }
}