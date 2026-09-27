//! Integration test suite for oxide-coordinator Actix Web endpoints using in-memory SurrealDB

use actix_web::{App, test, web};
use actix_web_prometheus::PrometheusMetricsBuilder;
use oxide_coordinator::{
    AppState,
    auth::AuthService,
    config::{AuthConfig, StorageBackendType, StorageConfig},
    handlers,
    storage::Storage,
};
use oxide_core::{MeshName, NodeId, OverlayPrefix};
use oxide_protocol::topics::{AclPolicy, NodeMetadata, RouteAdvertisement};
use std::sync::Arc;

async fn setup_test_app_state() -> AppState {
    let auth_config = AuthConfig::default();
    let auth_service = Arc::new(AuthService::new(&auth_config).expect("auth service init"));

    let storage_config = StorageConfig {
        backend: StorageBackendType::Memory,
        namespace: "test_mesh".into(),
        database: "test_db".into(),
        path: None,
        endpoint: None,
        username: None,
        password: None,
    };
    let storage = Arc::new(Storage::new_async(&storage_config).await.expect("storage init"));
    let mesh_name = MeshName::new("oxide-test-mesh").expect("valid mesh name");

    AppState {
        auth: auth_service,
        storage,
        mesh_name,
    }
}

#[actix_web::test]
async fn test_health_endpoints() {
    let app_state = setup_test_app_state().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .service(
                web::scope("/health")
                    .route("", web::get().to(handlers::health::health_check))
                    .route("/ready", web::get().to(handlers::health::readiness_check))
                    .route("/live", web::get().to(handlers::health::liveness_check)),
            ),
    )
    .await;

    // Test /health
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Test /health/ready
    let req = test::TestRequest::get().uri("/health/ready").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Test /health/live
    let req = test::TestRequest::get().uri("/health/live").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_nodes_and_routes_api() {
    let app_state = setup_test_app_state().await;

    // Seed storage with a node and a route
    let node_id = NodeId::new();
    let metadata = NodeMetadata {
        node_id,
        display_name: Some("test-edge-01".into()),
        os: "linux".into(),
        arch: "x86_64".into(),
        version: "0.1.0".into(),
        tags: vec!["gateway".into(), "edge".into()],
    };
    app_state.storage.set_node_metadata(&metadata).await.expect("set node");

    let prefix: OverlayPrefix = "100.64.0.0/24".parse().expect("valid prefix");
    let route = RouteAdvertisement {
        node_id,
        prefix,
        metric: 100,
        next_hop: None,
        communities: vec!["us-east".into()],
        timestamp: chrono::Utc::now().timestamp(),
    };
    app_state.storage.set_route(&route).await.expect("set route");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .service(web::scope("/api/v1").configure(handlers::configure_routes)),
    )
    .await;

    // 1. GET /api/v1/nodes
    let req = test::TestRequest::get().uri("/api/v1/nodes").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["total"], 1);
    assert_eq!(body["nodes"][0]["node_id"], node_id.to_string());
    assert_eq!(body["nodes"][0]["display_name"], "test-edge-01");

    // 2. GET /api/v1/nodes/{node_id}
    let req = test::TestRequest::get()
        .uri(&format!("/api/v1/nodes/{}", node_id))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // 3. GET /api/v1/routes
    let req = test::TestRequest::get().uri("/api/v1/routes").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["total"], 1);
    assert_eq!(body["routes"][0]["prefix"], "100.64.0.0/24");
    assert_eq!(body["routes"][0]["metric"], 100);
}

#[actix_web::test]
async fn test_acl_policy_api() {
    let app_state = setup_test_app_state().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .service(web::scope("/api/v1").configure(handlers::configure_routes)),
    )
    .await;

    // 1. GET /api/v1/acl (initially default or empty)
    let req = test::TestRequest::get().uri("/api/v1/acl").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // 2. PUT /api/v1/acl with new policy
    let policy = AclPolicy {
        version: 1,
        default_action: oxide_protocol::topics::AclAction::Deny,
        rules: vec![],
        updated_at: chrono::Utc::now().timestamp(),
    };

    let req = test::TestRequest::put()
        .uri("/api/v1/acl")
        .set_json(&policy)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // 3. GET /api/v1/acl again to verify updated policy
    let req = test::TestRequest::get().uri("/api/v1/acl").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["version"], 1);
}
