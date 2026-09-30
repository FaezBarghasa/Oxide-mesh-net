//! Integration tests for systemd-resolved, resolv.conf, and MagicDNS in oxide-dns

use oxide_dns::platform::{
    ResolvConfConfig, ResolvConfManager, SystemdResolvedConfig, SystemdResolvedController,
};
use oxide_dns::server::{MagicDnsConfig, MagicDnsServer};
use std::net::Ipv4Addr;
use std::path::PathBuf;

#[test]
fn test_systemd_resolved_and_resolv_conf_lifecycle() {
    // 1. Test systemd-resolved controller in dry-run
    let sys_cfg = SystemdResolvedConfig {
        interface: "oxide-test0".to_string(),
        dns_servers: vec!["100.100.100.100".to_string()],
        routing_domains: vec!["~mesh.oxide".to_string(), "~.".to_string()],
        dry_run: true,
    };
    let mut sys_ctrl = SystemdResolvedController::new(sys_cfg);
    assert!(!sys_ctrl.is_applied());
    assert!(sys_ctrl.apply().is_ok());
    assert!(sys_ctrl.is_applied());
    assert!(sys_ctrl.revert().is_ok());
    assert!(!sys_ctrl.is_applied());

    // 2. Test direct resolv.conf manager in dry-run
    let res_cfg = ResolvConfConfig {
        resolv_path: PathBuf::from("/tmp/oxide-dns-test.conf"),
        backup_path: PathBuf::from("/tmp/oxide-dns-test.conf.bak"),
        nameservers: vec!["100.100.100.100".to_string()],
        search_domains: vec!["mesh.oxide".to_string()],
        dry_run: true,
    };
    let mut res_mgr = ResolvConfManager::new(res_cfg);
    assert!(!res_mgr.backup_exists());
    assert!(res_mgr.apply().is_ok());
    assert!(res_mgr.backup_exists());
    assert!(res_mgr.restore().is_ok());
    assert!(!res_mgr.backup_exists());
}

#[test]
fn test_magic_dns_query_and_reverse_ptr_resolution() {
    let server = MagicDnsServer::new(MagicDnsConfig::default());

    let node_ip = Ipv4Addr::new(100, 64, 0, 10);
    let node_v6 = "fd00:0x1d:e::10".parse().ok();

    server.register_peer("frankfurt-gateway", node_ip, node_v6);
    assert_eq!(server.host_count(), 1);

    // Forward A query
    assert_eq!(
        server.lookup_a("frankfurt-gateway.mesh.oxide"),
        Some(node_ip)
    );
    // Case-insensitive query
    assert_eq!(
        server.lookup_a("FRANKFURT-GATEWAY.MESH.OXIDE."),
        Some(node_ip)
    );
    // Forward AAAA query
    assert_eq!(
        server.lookup_aaaa("frankfurt-gateway.mesh.oxide"),
        node_v6
    );

    // Reverse PTR query
    assert_eq!(
        server.lookup_ptr(&node_ip),
        Some("frankfurt-gateway.mesh.oxide".to_string())
    );

    // Cleanup
    server.unregister_peer("frankfurt-gateway");
    assert_eq!(server.host_count(), 0);
    assert_eq!(server.lookup_a("frankfurt-gateway.mesh.oxide"), None);
}
