//! ACME automatic TLS certificate lifecycle coordinator

use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tracing::info;

/// ACME Certificate Status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CertStatus {
    PendingChallenge,
    Valid,
    Expired,
    Revoked,
}

/// Managed ACME Certificate Record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedCertificate {
    pub domain: String,
    pub status: CertStatus,
    pub issued_at: u64,
    pub expires_at: u64,
    pub fingerprint_sha256: String,
}

/// ACME TLS Certificate Manager
pub struct AcmeManager {
    certificates: Arc<DashMap<String, ManagedCertificate>>,
    acme_directory_url: String,
}

impl AcmeManager {
    pub fn new(acme_directory_url: String) -> Self {
        Self {
            certificates: Arc::new(DashMap::new()),
            acme_directory_url,
        }
    }

    pub fn acme_directory_url(&self) -> &str {
        &self.acme_directory_url
    }

    /// Issue or renew certificate for domain
    pub fn issue_certificate(
        &self,
        domain: &str,
        fingerprint_sha256: &str,
    ) -> ManagedCertificate {
        info!("Issuing ACME TLS certificate for domain '{}'...", domain);
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_secs();

        // 90 days validity (Let's Encrypt standard)
        let expires_at = now + 90 * 86400;

        let cert = ManagedCertificate {
            domain: domain.to_lowercase(),
            status: CertStatus::Valid,
            issued_at: now,
            expires_at,
            fingerprint_sha256: fingerprint_sha256.to_string(),
        };

        self.certificates.insert(domain.to_lowercase(), cert.clone());
        cert
    }

    /// Get certificate by domain
    pub fn get_certificate(&self, domain: &str) -> Option<ManagedCertificate> {
        self.certificates.get(&domain.to_lowercase()).map(|c| c.clone())
    }

    /// Check if certificate needs renewal (e.g. less than 30 days remaining)
    pub fn needs_renewal(&self, domain: &str) -> bool {
        if let Some(cert) = self.get_certificate(domain) {
            let now = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or(Duration::ZERO)
                .as_secs();
            cert.expires_at.saturating_sub(now) < 30 * 86400
        } else {
            true
        }
    }

    pub fn total_certificates(&self) -> usize {
        self.certificates.len()
    }
}

impl Default for AcmeManager {
    fn default() -> Self {
        Self::new("https://acme-v02.api.letsencrypt.org/directory".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acme_certificate_issuance_and_lookup() {
        let manager = AcmeManager::default();
        assert_eq!(manager.total_certificates(), 0);

        let domain = "workstation.mesh.oxide.net";
        let fp = "sha256:7f9a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a";

        let cert = manager.issue_certificate(domain, fp);
        assert_eq!(cert.domain, domain);
        assert_eq!(cert.status, CertStatus::Valid);
        assert_eq!(manager.total_certificates(), 1);

        let fetched = manager.get_certificate(domain).unwrap();
        assert_eq!(fetched.fingerprint_sha256, fp);
        assert!(!manager.needs_renewal(domain));
    }
}
