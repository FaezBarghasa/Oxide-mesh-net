//! MagicDNS engine with split-horizon resolution

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use hickory_proto::op::{Message, MessageType, OpCode, ResponseCode};
use hickory_proto::rr::{Name, RData, Record, RecordType, RrKey};
use hickory_resolver::config::{ResolverConfig, ResolverOpts};
use hickory_resolver::TokioAsyncResolver;
use hickory_server::authority::{Catalog, ZoneType};
use hickory_server::server::{Request, RequestHandler, ResponseHandler, ResponseInfo};
use hickory_server::ServerFuture;
use crate::error::{DnsError, Result};
use oxide_core::{NodeId, OverlayIp, OverlayPrefix, MeshName};
use oxide_protocol::topics::DnsRecord;

/// MagicDNS configuration
#[derive(Debug, Clone)]
pub struct DnsConfig {
    /// Listen address for DNS queries
    pub listen_addr: SocketAddr,
    /// Mesh TLD (e.g., "oxide")
    pub mesh_tld: String,
    /// Upstream DNS resolvers (DoH)
    pub upstream_resolvers: Vec<String>,
    /// Enable DNSSEC validation
    pub dnssec: bool,
    /// Cache TTL
    pub cache_ttl: Duration,
    /// Enable split-horizon
    pub split_horizon: bool,
}

impl Default for DnsConfig {
    fn default() -> Self {
        Self {
            listen_addr: "100.100.100.100:53".parse().unwrap(),
            mesh_tld: "oxide".into(),
            upstream_resolvers: vec![
                "https://cloudflare-dns.com/dns-query".into(),
                "https://dns.google/dns-query".into(),
            ],
            dnssec: true,
            cache_ttl: Duration::from_secs(300),
            split_horizon: true,
        }
    }
}

/// MagicDNS engine
pub struct MagicDns {
    config: DnsConfig,
    catalog: Arc<dyn Catalog>,
    local_records: Arc<RwLock<HashMap<Name, Vec<Record>>>>,
    upstream_resolver: TokioAsyncResolver,
    mesh_name: MeshName,
}

impl MagicDns {
    pub async fn new(config: DnsConfig, mesh_name: MeshName) -> Result<Self> {
        // Build upstream resolver with DoH
        let resolver_config = ResolverConfig::from_parts(
            None,
            vec![],
            config.upstream_resolvers.iter()
                .filter_map(|url| url.parse().ok())
                .collect(),
        );
        let resolver_opts = ResolverOpts {
            validate: config.dnssec,
            ..Default::default()
        };
        let upstream_resolver = TokioAsyncResolver::tokio(resolver_config, resolver_opts);

        // Create in-memory catalog for mesh zones
        let catalog = Arc::new(MemoryCatalog::new());

        // Create local records store
        let local_records = Arc::new(RwLock::new(HashMap::new()));

        Ok(Self {
            config,
            catalog,
            local_records,
            upstream_resolver,
            mesh_name,
        })
    }

    /// Start the DNS server
    pub async fn start(&self) -> Result<()> {
        let mut server = ServerFuture::new(self.create_request_handler());
        server.register_socket(self.config.listen_addr.into())
            .map_err(|e| DnsError::Io(e))?;

        tokio::spawn(async move {
            if let Err(e) = server.block_until_done().await {
                tracing::error!("DNS server error: {}", e);
            }
        });

        tracing::info!("MagicDNS listening on {}", self.config.listen_addr);
        Ok(())
    }

    fn create_request_handler(&self) -> impl RequestHandler {
        MagicDnsRequestHandler {
            config: self.config.clone(),
            catalog: self.catalog.clone(),
            local_records: self.local_records.clone(),
            upstream_resolver: self.upstream_resolver.clone(),
            mesh_name: self.mesh_name.clone(),
        }
    }

    /// Add a mesh node record
    pub async fn add_node_record(&self, name: &str, ip: OverlayIp, ttl: u32) -> Result<()> {
        let fqdn = format!("{}.{}.", name, self.config.mesh_tld);
        let name: Name = fqdn.parse().map_err(|e| DnsError::Protocol(e.to_string()))?;

        let rdata = match ip {
            OverlayIp::V4(addr) => RData::A(addr),
            OverlayIp::V6(addr) => RData::AAAA(addr),
        };

        let record = Record::from_rdata(name.clone(), ttl, rdata);
        self.local_records.write().await.entry(name).or_default().push(record);
        
        // Update catalog
        self.catalog.add_zone(/* zone */);
        
        Ok(())
    }

    /// Remove a mesh node record
    pub async fn remove_node_record(&self, name: &str) -> Result<()> {
        let fqdn = format!("{}.{}.", name, self.config.mesh_tld);
        let name: Name = fqdn.parse().map_err(|e| DnsError::Protocol(e.to_string()))?;
        self.local_records.write().await.remove(&name);
        Ok(())
    }

    /// Add a subnet router route
    pub async fn add_subnet_route(&self, prefix: OverlayPrefix, router_name: &str) -> Result<()> {
        // Add NS record for subnet delegation
        Ok(())
    }

    /// Get statistics
    pub async fn stats(&self) -> DnsStats {
        let local_count = self.local_records.read().await.len();
        DnsStats {
            local_records: local_count,
            cache_entries: 0, // Would track cache
            upstream_queries: 0,
        }
    }
}

/// DNS statistics
#[derive(Debug, Clone)]
pub struct DnsStats {
    pub local_records: usize,
    pub cache_entries: usize,
    pub upstream_queries: u64,
}

/// In-memory catalog for dynamic zones
struct MemoryCatalog {
    zones: RwLock<HashMap<Name, ZoneData>>,
}

struct ZoneData {
    records: Vec<Record>,
    zone_type: ZoneType,
}

impl MemoryCatalog {
    fn new() -> Self {
        Self {
            zones: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl Catalog for MemoryCatalog {
    async fn zone(&self, name: &Name) -> Option<Arc<dyn hickory_server::authority::Authority>> {
        // Return authority for zone
        None
    }

    async fn zones(&self) -> Vec<Name> {
        self.zones.read().await.keys().cloned().collect()
    }
}

/// MagicDNS request handler
struct MagicDnsRequestHandler {
    config: DnsConfig,
    catalog: Arc<dyn Catalog>,
    local_records: Arc<RwLock<HashMap<Name, Vec<Record>>>>,
    upstream_resolver: TokioAsyncResolver,
    mesh_name: MeshName,
}

#[async_trait::async_trait]
impl RequestHandler for MagicDnsRequestHandler {
    async fn handle_request<R: ResponseHandler>(
        &self,
        request: &Request,
        response_handler: R,
    ) -> ResponseInfo {
        let query = request.query();
        let name = query.name().clone();
        let record_type = query.query_type();

        // Check if it's a mesh domain
        if self.is_mesh_domain(&name) {
            return self.handle_mesh_query(request, response_handler, &name, record_type).await;
        }

        // Check if it's a subnet router domain
        if self.is_subnet_domain(&name) {
            return self.handle_subnet_query(request, response_handler, &name, record_type).await;
        }

        // Forward to upstream resolver
        self.forward_upstream(request, response_handler).await
    }
}

impl MagicDnsRequestHandler {
    fn is_mesh_domain(&self, name: &Name) -> bool {
        name.to_string().ends_with(&format!("{}.", self.config.mesh_tld))
    }

    fn is_subnet_domain(&self, name: &Name) -> bool {
        // Check if domain is delegated to a subnet router
        false
    }

    async fn handle_mesh_query<R: ResponseHandler>(
        &self,
        request: &Request,
        mut response_handler: R,
        name: &Name,
        record_type: RecordType,
    ) -> ResponseInfo {
        // Check local records first
        if let Some(records) = self.local_records.read().await.get(name) {
            let mut response = Message::new();
            response.set_id(request.id());
            response.set_message_type(MessageType::Response);
            response.set_op_code(OpCode::Query);
            response.set_response_code(ResponseCode::NoError);
            
            for record in records {
                if record.record_type() == record_type || record_type == RecordType::ANY {
                    response.add_answer(record.clone());
                }
            }

            return response_handler.send_response(response).await.unwrap();
        }

        // Not found locally, try upstream if split-horizon allows
        if self.config.split_horizon {
            self.forward_upstream(request, response_handler).await
        } else {
            let mut response = Message::new();
            response.set_id(request.id());
            response.set_message_type(MessageType::Response);
            response.set_response_code(ResponseCode::NXDomain);
            response_handler.send_response(response).await.unwrap()
        }
    }

    async fn handle_subnet_query<R: ResponseHandler>(
        &self,
        request: &Request,
        response_handler: R,
        name: &Name,
        record_type: RecordType,
    ) -> ResponseInfo {
        // Would forward to subnet router's DNS
        self.forward_upstream(request, response_handler).await
    }

    async fn forward_upstream<R: ResponseHandler>(
        &self,
        request: &Request,
        response_handler: R,
    ) -> ResponseInfo {
        let query = request.query();
        let name = query.name().clone();
        let record_type = query.query_type();

        match self.upstream_resolver.lookup(name, record_type).await {
            Ok(lookup) => {
                let mut response = Message::new();
                response.set_id(request.id());
                response.set_message_type(MessageType::Response);
                response.set_op_code(OpCode::Query);
                response.set_response_code(ResponseCode::NoError);
                
                for record in lookup.iter() {
                    response.add_answer(record.clone());
                }

                response_handler.send_response(response).await.unwrap()
            }
            Err(e) => {
                let mut response = Message::new();
                response.set_id(request.id());
                response.set_message_type(MessageType::Response);
                response.set_response_code(ResponseCode::ServFail);
                response_handler.send_response(response).await.unwrap()
            }
        }
    }
}

/// Split-horizon resolver for client-side DNS
pub struct SplitHorizonResolver {
    mesh_dns: SocketAddr,
    upstream_resolver: TokioAsyncResolver,
    mesh_tld: String,
}

impl SplitHorizonResolver {
    pub fn new(mesh_dns: SocketAddr, upstream_resolvers: Vec<String>, mesh_tld: String) -> Result<Self> {
        let resolver_config = ResolverConfig::from_parts(
            None,
            vec![],
            upstream_resolvers.iter()
                .filter_map(|url| url.parse().ok())
                .collect(),
        );
        let upstream_resolver = TokioAsyncResolver::tokio(resolver_config, ResolverOpts::default());

        Ok(Self {
            mesh_dns,
            upstream_resolver,
            mesh_tld,
        })
    }

    pub async fn lookup(&self, name: &str, record_type: RecordType) -> Result<Vec<Record>> {
        let name: Name = name.parse().map_err(|e| DnsError::Protocol(e.to_string()))?;

        // Check if mesh domain
        if name.to_string().ends_with(&format!("{}.", self.mesh_tld)) {
            // Query mesh DNS directly
            // Would use hickory_resolver with custom config
        }

        // Otherwise use upstream
        let lookup = self.upstream_resolver.lookup(name, record_type).await
            .map_err(|e| DnsError::UpstreamError(e.to_string()))?;
        
        Ok(lookup.iter().cloned().collect())
    }
}