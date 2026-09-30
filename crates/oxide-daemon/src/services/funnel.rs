//! Oxide Funnel & Serve public ingress reverse proxy engine

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::atomic::AtomicU64;
use std::sync::Arc;
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

    pub fn stats(&self) -> &Arc<FunnelStats> {
        &self.stats
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
        assert_eq!(found.local_target, "127.0.0.1:8080".parse::<SocketAddr>().unwrap());
        assert!(found.allow_funnel);

        assert!(service.unregister_service("dev-api"));
        assert_eq!(service.list_services().len(), 0);
    }
}
