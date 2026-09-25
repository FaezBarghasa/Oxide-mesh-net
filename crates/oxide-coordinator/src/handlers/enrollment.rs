//! Enrollment handlers

use actix_web::{web, HttpResponse, Responder, post, get};
use serde::{Deserialize, Serialize};
use crate::{AppState, error::{CoordinatorError, Result}};
use oxide_core::{NodeId, MeshName, NodeCapabilities, Endpoint};
use oxide_crypto::keys::{DeviceIdentityPublicKey, SessionPublicKey};

#[derive(Deserialize)]
pub struct EnrollRequest {
    pub token: String,
    pub identity_pub: String,      // Base58 encoded DeviceIdentityPublicKey
    pub session_pub: String,       // Base58 encoded SessionPublicKey
    pub endpoints: Vec<Endpoint>,
    pub capabilities: NodeCapabilities,
    pub display_name: Option<String>,
}

#[derive(Serialize)]
pub struct EnrollResponse {
    pub node_id: NodeId,
    pub mesh_name: MeshName,
    pub overlay_ips: Vec<String>,
    pub token: String,             // JWT token for subsequent auth
    pub config: EnrollConfig,
}

#[derive(Serialize)]
pub struct EnrollConfig {
    pub mqtt_endpoints: Vec<String>,
    pub stun_servers: Vec<String>,
    pub dns_servers: Vec<String>,
    pub mtu: u16,
    pub keepalive_interval: u64,
}

#[post("/enroll")]
pub async fn enroll(
    data: web::Data<AppState>,
    req: web::Json<EnrollRequest>,
) -> Result<impl Responder> {
    // Validate enrollment token
    let (mesh_name, capabilities) = data.auth.validate_enrollment_token(&req.token).await?;

    // Parse public keys
    let identity_pub = DeviceIdentityPublicKey::from_str(&req.identity_pub)
        .map_err(|_| CoordinatorError::Auth("Invalid identity public key".into()))?;
    let session_pub = SessionPublicKey::from_str(&req.session_pub)
        .map_err(|_| CoordinatorError::Auth("Invalid session public key".into()))?;

    // Generate node ID
    let node_id = NodeId::new();

    // Assign overlay IPs (simplified - would use IPAM in reality)
    let overlay_ips = vec!["100.64.0.1".to_string()]; // Placeholder

    // Generate JWT token
    let jwt_token = data.auth.generate_token(node_id, &mesh_name.parse().unwrap(), capabilities)?;

    // Store node metadata
    let metadata = oxide_protocol::topics::NodeMetadata {
        node_id,
        display_name: req.display_name.clone(),
        os: "unknown".into(),
        arch: "unknown".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        tags: vec![],
    };
    data.storage.set_node_metadata(&metadata).await?;

    // Store identity key
    // Would store in storage

    let response = EnrollResponse {
        node_id,
        mesh_name: mesh_name.parse().unwrap(),
        overlay_ips,
        token: jwt_token,
        config: EnrollConfig {
            mqtt_endpoints: vec!["mqtt://coordinator:1883".into()],
            stun_servers: vec!["stun://stun.l.google.com:19302".into()],
            dns_servers: vec!["100.100.100.100".into()],
            mtu: 1500,
            keepalive_interval: 10,
        },
    };

    Ok(HttpResponse::Ok().json(response))
}

#[derive(Deserialize)]
pub struct ReenrollRequest {
    pub node_id: NodeId,
    pub session_pub: String,
}

#[post("/reenroll")]
pub async fn reenroll(
    data: web::Data<AppState>,
    req: web::Json<ReenrollRequest>,
) -> Result<impl Responder> {
    // Validate node exists
    let metadata = data.storage.get_node_metadata(&req.node_id).await?
        .ok_or(CoordinatorError::Auth("Node not found".into()))?;

    // Parse new session key
    let session_pub = SessionPublicKey::from_str(&req.session_pub)
        .map_err(|_| CoordinatorError::Auth("Invalid session public key".into()))?;

    // Generate new JWT token
    let jwt_token = data.auth.generate_token(
        req.node_id,
        &data.mesh_name,
        vec!["node".into()],
    )?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "token": jwt_token,
    })))
}

#[get("/enrollment-token")]
pub async fn create_enrollment_token(
    data: web::Data<AppState>,
    query: web::Query<CreateTokenQuery>,
) -> Result<impl Responder> {
    let token = data.auth.create_enrollment_token(
        data.mesh_name.to_string(),
        query.capabilities.clone().unwrap_or_default(),
        query.ttl.map(std::time::Duration::from_secs),
    ).await;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "token": token })))
}

#[derive(Deserialize)]
pub struct CreateTokenQuery {
    pub capabilities: Option<Vec<String>>,
    pub ttl: Option<u64>,
}

pub fn configure_enrollment_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(enroll)
       .service(reenroll)
       .service(create_enrollment_token);
}