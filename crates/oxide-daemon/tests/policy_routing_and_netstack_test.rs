//! Integration tests for Policy Routing, Userspace Netstack, and Funnel Service

use ipnet::Ipv4Net;
use oxide_daemon::platform::{PolicyRoutingConfig, PolicyRoutingManager};
use oxide_daemon::services::funnel::{FunnelService, ServeRule, ServiceProtocol};
use oxide_daemon::services::netstack::{
    TargetAddress, UserspaceNetstackServer, SOCKS5_ATYP_DOMAIN, SOCKS5_AUTH_NONE,
    SOCKS5_CMD_CONNECT, SOCKS5_VERSION,
};
use std::net::Ipv4Addr;
use std::str::FromStr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[test]
fn test_policy_routing_exit_node_and_subnet_lifecycle() {
    let config = PolicyRoutingConfig {
        table_id: 51820,
        fwmark: 0x51820,
        tun_interface: "oxide-test0".to_string(),
        overlay_ip: Ipv4Addr::new(100, 64, 0, 42),
        allow_lan_access: true,
        dry_run: true,
    };

    let mut mgr = PolicyRoutingManager::new(config);
    assert_eq!(mgr.applied_rules_count(), 0);

    // 1. Setup policy rules
    assert!(mgr.setup_policy_rules().is_ok());
    assert_eq!(mgr.applied_rules_count(), 2);

    // 2. Enable exit node routing
    let exit_gw = Ipv4Addr::new(100, 64, 0, 1);
    assert!(mgr.enable_exit_node(exit_gw).is_ok());
    assert_eq!(mgr.active_exit_node(), Some(exit_gw));

    // 3. Setup Subnet Masquerading
    let subnet = Ipv4Net::from_str("10.200.0.0/16").unwrap();
    assert!(mgr.setup_subnet_masquerade(subnet, "eth0").is_ok());

    // 4. Teardown
    assert!(mgr.disable_exit_node().is_ok());
    assert_eq!(mgr.active_exit_node(), None);

    assert!(mgr.teardown_subnet_masquerade(subnet, "eth0").is_ok());
    assert!(mgr.teardown_policy_rules().is_ok());
    assert_eq!(mgr.applied_rules_count(), 0);
}

#[tokio::test]
async fn test_userspace_socks5_greeting_and_request_parsing() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let server_addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        // 1. Handle greeting
        UserspaceNetstackServer::handle_socks5_greeting(&mut stream)
            .await
            .expect("greeting handling");
        // 2. Handle request
        let target = UserspaceNetstackServer::handle_socks5_request(&mut stream)
            .await
            .expect("request handling");

        assert_eq!(
            target,
            TargetAddress::Domain("gateway.mesh.oxide".to_string(), 443)
        );
    });

    let mut client = TcpStream::connect(server_addr).await.unwrap();

    // 1. Send SOCKS5 Greeting (Version 5, 1 Method, Method 0 = No Auth)
    client.write_all(&[SOCKS5_VERSION, 1, SOCKS5_AUTH_NONE]).await.unwrap();
    let mut greeting_resp = [0u8; 2];
    client.read_exact(&mut greeting_resp).await.unwrap();
    assert_eq!(greeting_resp, [SOCKS5_VERSION, SOCKS5_AUTH_NONE]);

    // 2. Send SOCKS5 Connect Request to "gateway.mesh.oxide:443"
    let domain = "gateway.mesh.oxide";
    let mut req = vec![
        SOCKS5_VERSION,
        SOCKS5_CMD_CONNECT,
        0x00, // Reserved
        SOCKS5_ATYP_DOMAIN,
        domain.len() as u8,
    ];
    req.extend_from_slice(domain.as_bytes());
    req.extend_from_slice(&443u16.to_be_bytes());

    client.write_all(&req).await.unwrap();

    let mut req_resp = [0u8; 10];
    client.read_exact(&mut req_resp).await.unwrap();
    assert_eq!(req_resp[0], SOCKS5_VERSION);
    assert_eq!(req_resp[1], 0x00); // SUCCESS
}

#[test]
fn test_funnel_and_serve_registration_and_lookup() {
    let mut service = FunnelService::new();

    let rule = ServeRule {
        service_name: "prod-dashboard".to_string(),
        local_target: "127.0.0.1:3000".parse().unwrap(),
        protocol: ServiceProtocol::Http,
        mesh_alias: "dash.mesh.oxide".to_string(),
        allow_funnel: true,
        public_hostname: Some("dash.funnel.oxide.net".to_string()),
    };

    service.register_service(rule.clone());
    assert_eq!(service.list_services().len(), 1);

    let fetched = service.find_by_alias("dash.mesh.oxide").unwrap();
    assert_eq!(fetched.service_name, "prod-dashboard");
    assert!(fetched.allow_funnel);

    assert!(service.unregister_service("prod-dashboard"));
    assert_eq!(service.list_services().len(), 0);
}
