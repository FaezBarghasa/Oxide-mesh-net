//! ACL management handlers

use actix_web::{web, HttpResponse, Responder, get, put, delete};
use serde::{Deserialize, Serialize};
use crate::{AppState, error::{CoordinatorError, Result}};
use oxide_core::{NodeId, MeshName, OverlayPrefix, OverlayIp};
use oxide_crypto::keys::KeyFingerprint;
use oxide_protocol::topics::{AclAction, AclDirection};

#[derive(Deserialize)]
pub struct UpdateAclRequest {
    pub default_action: AclAction,
    pub rules: Vec<AclRuleRequest>,
}

#[derive(Deserialize, Serialize)]
pub struct AclRuleRequest {
    pub id: String,
    pub action: AclAction,
    pub src_identities: Vec<KeyFingerprint>,
    pub dst_prefixes: Vec<OverlayPrefix>,
    pub protocols: Vec<u8>,
    pub port_ranges: Vec<(u16, u16)>,
    pub direction: AclDirection,
    pub log: bool,
    pub priority: u32,
}

#[derive(Serialize)]
pub struct AclPolicyResponse {
    pub version: u64,
    pub default_action: AclAction,
    pub rules: Vec<AclRuleResponse>,
    pub updated_at: i64,
}

#[derive(Serialize)]
pub struct AclRuleResponse {
    pub id: String,
    pub action: AclAction,
    pub src_identities: Vec<KeyFingerprint>,
    pub dst_prefixes: Vec<OverlayPrefix>,
    pub protocols: Vec<u8>,
    pub port_ranges: Vec<(u16, u16)>,
    pub direction: AclDirection,
    pub log: bool,
    pub priority: u32,
}

#[get("/acl")]
pub async fn get_acl(data: web::Data<AppState>) -> Result<impl Responder> {
    let policy = data.storage.get_acl_policy().await?
        .unwrap_or_default();

    let response = AclPolicyResponse {
        version: policy.version,
        default_action: policy.default_action,
        rules: policy.rules.into_iter().map(|r| AclRuleResponse {
            id: r.id,
            action: r.action,
            src_identities: r.src_identities,
            dst_prefixes: r.dst_prefixes,
            protocols: r.protocols,
            port_ranges: r.port_ranges,
            direction: r.direction,
            log: r.log,
            priority: r.priority,
        }).collect(),
        updated_at: policy.timestamp,
    };

    Ok(HttpResponse::Ok().json(response))
}

#[put("/acl")]
pub async fn update_acl(
    data: web::Data<AppState>,
    req: web::Json<UpdateAclRequest>,
) -> Result<impl Responder> {
    // Validate rules
    for rule in &req.rules {
        if rule.id.is_empty() {
            return Err(CoordinatorError::Config("Rule ID cannot be empty".into()));
        }
    }

    // Get current version
    let current = data.storage.get_acl_policy().await?.unwrap_or_default();
    let new_version = current.version + 1;

    // Build new policy
    let policy = oxide_protocol::topics::AclPolicy {
        version: new_version,
        default_action: req.default_action,
        rules: req.rules.iter().map(|r| oxide_protocol::topics::AclRule {
            id: r.id.clone(),
            action: r.action,
            src_identities: r.src_identities.clone(),
            dst_prefixes: r.dst_prefixes.clone(),
            protocols: r.protocols.clone(),
            port_ranges: r.port_ranges.clone(),
            direction: r.direction,
            log: r.log,
            priority: r.priority,
        }).collect(),
        timestamp: chrono::Utc::now().timestamp(),
        signature: oxide_crypto::keys::DeviceSignature::from_bytes(&[0; 64]).unwrap(),
    };

    data.storage.set_acl_policy(&policy).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "version": new_version })))
}

#[delete("/acl")]
pub async fn delete_acl(data: web::Data<AppState>) -> Result<impl Responder> {
    let policy = oxide_protocol::topics::AclPolicy {
        version: 0,
        default_action: AclAction::Allow,
        rules: vec![],
        timestamp: chrono::Utc::now().timestamp(),
        signature: oxide_crypto::keys::DeviceSignature::from_bytes(&[0; 64]).unwrap(),
    };
    data.storage.set_acl_policy(&policy).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub fn configure_acl_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_acl)
       .service(update_acl)
       .service(delete_acl);
}