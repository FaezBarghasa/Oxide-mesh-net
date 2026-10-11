//! Oxide Funnel & Serve public ingress reverse proxy engine

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use tracing::info;

/// Service protocol type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceProtocol {
    Http,
    Https,
    Tcp,
    TlsTerminated,
}

/// Local service exposure configuration (Tailscale Serve equivalent)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServeRule {
    pub service_name: String,
    pub local_target: SocketAddr,
    pub protocol: ServiceProtocol,
    pub mesh_alias: String,
    pub allow_funnel: bool,
    pub public_hostname: Option<String>,
}

/// Statistics for Funnel & Serve proxy
#[derive(Debug, Default)]
pub struct FunnelStats {
    pub total_requests: AtomicU64,
    pub bytes_forwarded: AtomicU64,
    pub acme_certificates_active: AtomicU64,
}

/// Funnel & Serve Service Manager
pub struct FunnelService {
    rules: Vec<ServeRule>,
    stats: Arc<FunnelStats>,
}

impl FunnelService {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            stats: Arc::new(FunnelStats::default()),
        }
    }

    /// Add or update a local serve rule
    pub fn register_service(&mut self, rule: ServeRule) {
        info!(
            "Registering Oxide-Serve service '{}' on target {} (Mesh: {}, Funnel: {})",
            rule.service_name, rule.local_target, rule.mesh_alias, rule.allow_funnel
        );

        if let Some(pos) = self
            .rules
            .iter()
            .position(|r| r.service_name == rule.service_name)
        {
            self.rules[pos] = rule;
        } else {
            self.rules.push(rule);
        }
    }

    /// Unregister a service rule
    pub fn unregister_service(&mut self, service_name: &str) -> bool {
        let initial_len = self.rules.len();
        self.rules.retain(|r| r.service_name != service_name);
        self.rules.len() < initial_len
    }

    /// List all active serve rules
    pub fn list_services(&self) -> &[ServeRule] {
        &self.rules
    }

    /// Find serve rule by mesh alias domain
    pub fn find_by_alias(&self, alias: &str) -> Option<&ServeRule> {
        self.rules.iter().find(|r| r.mesh_alias == alias)
    }

    /// Spawn a local proxy listener forwarding incoming requests on bind_addr to local_target
    pub async fn start_proxy_worker(
        &self,
        bind_addr: SocketAddr,
        target_addr: SocketAddr,
    ) -> Result<(), String> {
        let listener = tokio::net::TcpListener::bind(bind_addr)
            .await
            .map_err(|e| format!("Failed to bind funnel proxy listener {}: {}", bind_addr, e))?;

        let stats = self.stats.clone();
        tokio::spawn(async move {
            while let Ok((mut client, _)) = listener.accept().await {
                let stats_clone = stats.clone();
                tokio::spawn(async move {
                    if let Ok(mut target) = tokio::net::TcpStream::connect(target_addr).await {
                        stats_clone
                            .total_requests
                            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        let (mut cr, mut cw) = client.split();
                        let (mut tr, mut tw) = target.split();
                        let c_to_t = tokio::io::copy(&mut cr, &mut tw);
                        let t_to_c = tokio::io::copy(&mut tr, &mut cw);
                        let _ = tokio::select! {
                            r1 = c_to_t => r1,
                            r2 = t_to_c => r2,
                        };
                    }
                });
            }
        });

        Ok(())
    }

    pub fn stats(&self) -> &Arc<FunnelStats> {
        &self.stats
    }
}

/// Dynamic Virtual Host HTTP Reverse Proxy
///
/// Inspects incoming HTTP Host headers and SNI indicators to route requests for distinct
/// domains / subdomains (e.g. `sub.domain.com`, `sub3.domain.com`) to specific backend ports (e.g. 8282, 6543).
#[derive(Debug, Clone)]
pub struct VirtualHostRouter {
    /// Domain/subdomain (lowercased) -> local destination port
    routes: Arc<tokio::sync::RwLock<std::collections::HashMap<String, SocketAddr>>>,
    bind_addr: SocketAddr,
}

impl VirtualHostRouter {
    pub fn new(bind_addr: SocketAddr) -> Self {
        Self {
            routes: Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
            bind_addr,
        }
    }

    /// Add or update a domain/subdomain mapping to a target port
    pub async fn set_route(&self, hostname: &str, target_addr: SocketAddr) {
        let mut routes = self.routes.write().await;
        routes.insert(hostname.to_ascii_lowercase(), target_addr);
        info!(
            "Mapped virtual host '{}' -> target {}",
            hostname, target_addr
        );
    }

    /// Remove a domain/subdomain mapping
    pub async fn remove_route(&self, hostname: &str) -> bool {
        let mut routes = self.routes.write().await;
        routes.remove(&hostname.to_ascii_lowercase()).is_some()
    }

    /// Get target address for a given domain/subdomain
    pub async fn resolve_host(&self, hostname: &str) -> Option<SocketAddr> {
        let routes = self.routes.read().await;
        let lower = hostname.to_ascii_lowercase();
        // Exact match first, then wildcards (e.g. *.domain.com)
        if let Some(target) = routes.get(&lower) {
            return Some(*target);
        }
        for (pattern, target) in routes.iter() {
            if pattern.starts_with("*.") {
                let suffix = &pattern[1..]; // .domain.com
                if lower.ends_with(suffix) {
                    return Some(*target);
                }
            }
        }
        None
    }

    /// Extract Host header from raw HTTP request buffer
    pub fn extract_host_header(buffer: &[u8]) -> Option<String> {
        let text = std::str::from_utf8(buffer).ok()?;
        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.to_ascii_lowercase().starts_with("host:") {
                let host_val = trimmed["host:".len()..].trim();
                // Strip optional port in Host header (e.g. sub.domain.com:80 -> sub.domain.com)
                let clean_host = host_val.split(':').next().unwrap_or(host_val);
                return Some(clean_host.to_string());
            }
        }
        None
    }

    /// Start listening and dispatching incoming connections based on Host headers
    pub async fn start(self: Arc<Self>) -> Result<(), String> {
        let listener = tokio::net::TcpListener::bind(self.bind_addr)
            .await
            .map_err(|e| {
                format!(
                    "Failed to bind virtual host router on {}: {}",
                    self.bind_addr, e
                )
            })?;

        info!("Virtual Host HTTP router listening on {}", self.bind_addr);
        let this = self.clone();

        tokio::spawn(async move {
            use tokio::io::AsyncWriteExt;
            while let Ok((mut client, _)) = listener.accept().await {
                let router = this.clone();
                tokio::spawn(async move {
                    let mut peek_buf = [0u8; 4096];
                    let n = match client.peek(&mut peek_buf).await {
                        Ok(n) if n > 0 => n,
                        _ => return,
                    };

                    if let Some(host) = Self::extract_host_header(&peek_buf[..n])
                        && let Some(target_addr) = router.resolve_host(&host).await
                        && let Ok(mut backend) = tokio::net::TcpStream::connect(target_addr).await
                    {
                        let (mut cr, mut cw) = client.split();
                        let (mut br, mut bw) = backend.split();
                        let c_to_b = tokio::io::copy(&mut cr, &mut bw);
                        let b_to_c = tokio::io::copy(&mut br, &mut cw);
                        let _ = tokio::select! {
                            r1 = c_to_b => r1,
                            r2 = b_to_c => r2,
                        };
                        return;
                    }

                    // Return 404/502 Bad Gateway if host is unrecognized
                    let not_found = b"HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\nOxide Mesh Net: Virtual Host Not Found\r\n";
                    let _ = client.write_all(not_found).await;
                });
            }
        });

        Ok(())
    }
}

impl Default for FunnelService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_funnel_service_registration_and_lookup() {
        let mut service = FunnelService::new();
        assert_eq!(service.list_services().len(), 0);

        let rule = ServeRule {
            service_name: "dev-api".to_string(),
            local_target: "127.0.0.1:8080".parse().unwrap(),
            protocol: ServiceProtocol::Http,
            mesh_alias: "api.mesh.oxide".to_string(),
            allow_funnel: true,
            public_hostname: Some("api.funnel.oxide.net".to_string()),
        };

        service.register_service(rule.clone());
        assert_eq!(service.list_services().len(), 1);

        let found = service.find_by_alias("api.mesh.oxide").unwrap();
        assert_eq!(found.service_name, "dev-api");
        assert_eq!(
            found.local_target,
            "127.0.0.1:8080".parse::<SocketAddr>().unwrap()
        );
        assert!(found.allow_funnel);

        assert!(service.unregister_service("dev-api"));
        assert_eq!(service.list_services().len(), 0);
    }

    #[tokio::test]
    async fn test_virtual_host_router_matching() {
        let router = VirtualHostRouter::new("0.0.0.0:80".parse().unwrap());
        router
            .set_route("sub.domain.com", "127.0.0.1:8282".parse().unwrap())
            .await;
        router
            .set_route("sub3.domain.com", "127.0.0.1:6543".parse().unwrap())
            .await;
        router
            .set_route("*.corp.local", "127.0.0.1:9000".parse().unwrap())
            .await;

        // Exact match testing
        assert_eq!(
            router.resolve_host("sub.domain.com").await,
            Some("127.0.0.1:8282".parse().unwrap())
        );
        assert_eq!(
            router.resolve_host("sub3.domain.com").await,
            Some("127.0.0.1:6543".parse().unwrap())
        );

        // Wildcard match testing
        assert_eq!(
            router.resolve_host("app.corp.local").await,
            Some("127.0.0.1:9000".parse().unwrap())
        );
        assert_eq!(
            router.resolve_host("nested.app.corp.local").await,
            Some("127.0.0.1:9000".parse().unwrap())
        );

        // Non-existent route
        assert_eq!(router.resolve_host("unknown.domain.com").await, None);
    }

    #[test]
    fn test_extract_host_header() {
        let raw_http = b"GET /api/v1/status HTTP/1.1\r\nHost: sub.domain.com:8282\r\nUser-Agent: curl/7.81.0\r\n\r\n";
        let extracted = VirtualHostRouter::extract_host_header(raw_http);
        assert_eq!(extracted.as_deref(), Some("sub.domain.com"));

        let raw_http_no_port =
            b"POST /submit HTTP/1.1\r\nhost: sub3.domain.com\r\nContent-Length: 0\r\n\r\n";
        let extracted2 = VirtualHostRouter::extract_host_header(raw_http_no_port);
        assert_eq!(extracted2.as_deref(), Some("sub3.domain.com"));
    }
}
