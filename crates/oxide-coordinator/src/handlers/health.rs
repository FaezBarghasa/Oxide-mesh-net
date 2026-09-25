//! Health check handlers

use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use crate::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub mesh: String,
}

#[derive(Serialize)]
pub struct ReadinessResponse {
    pub ready: bool,
    pub checks: Vec<CheckResult>,
}

#[derive(Serialize)]
pub struct CheckResult {
    pub name: String,
    pub status: String,
    pub message: Option<String>,
}

pub async fn health_check(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "healthy".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        mesh: data.mesh_name.to_string(),
    })
}

pub async fn readiness_check(data: web::Data<AppState>) -> impl Responder {
    // Check storage connectivity
    let storage_ok = data.storage.list_nodes().await.is_ok();
    
    // Check broker connectivity
    let broker_ok = data.broker.is_connected();

    let checks = vec![
        CheckResult {
            name: "storage".into(),
            status: if storage_ok { "ok".into() } else { "failed".into() },
            message: None,
        },
        CheckResult {
            name: "broker".into(),
            status: if broker_ok { "ok".into() } else { "failed".into() },
            message: None,
        },
    ];

    let ready = checks.iter().all(|c| c.status == "ok");
    let status_code = if ready { 200 } else { 503 };

    HttpResponse::build(status_code.into()).json(ReadinessResponse { ready, checks })
}

pub async fn liveness_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "alive": true }))
}