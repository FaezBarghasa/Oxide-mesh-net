//! Integration tests for ACME TLS certificate lifecycle in oxide-coordinator

use oxide_coordinator::acme::{AcmeManager, CertStatus};

#[test]
fn test_acme_manager_issuance_and_status() {
    let manager = AcmeManager::default();
    assert_eq!(manager.total_certificates(), 0);

    let domain = "eu-gateway.mesh.oxide.net";
    let sha256_fp = "sha256:4a9f1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a";

    let cert = manager.issue_certificate(domain, sha256_fp);
    assert_eq!(cert.domain, domain);
    assert_eq!(cert.status, CertStatus::Valid);
    assert_eq!(cert.fingerprint_sha256, sha256_fp);
    assert_eq!(manager.total_certificates(), 1);

    let fetched = manager.get_certificate(domain).expect("certificate exists");
    assert_eq!(fetched.fingerprint_sha256, sha256_fp);
    assert!(!manager.needs_renewal(domain));
}
