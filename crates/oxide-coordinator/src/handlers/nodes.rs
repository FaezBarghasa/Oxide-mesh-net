use crate::{
    AppState,
    error::{CoordinatorError, Result},
};
use actix_web::{HttpResponse, Responder, delete, get, web};
use oxide_core::NodeId;
use serde::Serialize;
use std::str::FromStr;

#[derive(Serialize)]
pub struct NodeListResponse {
    pub nodes: Vec<NodeInfo>,
    pub total: usize,
}

#[derive(Serialize)]
pub struct NodeInfo {
    pub node_id: NodeId,
    pub display_name: Option<String>,
    pub os: String,
    pub arch: String,
    pub version: String,
    pub last_seen: Option<i64>,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub tags: Vec<String>,
}

#[get("/nodes")]
pub async fn list_nodes(data: web::Data<AppState>) -> Result<impl Responder> {
    let nodes = data.storage.list_nodes().await?;
    let node_infos: Vec<NodeInfo> = nodes
        .into_iter()
        .map(|n| NodeInfo {
            node_id: n.node_id,
            display_name: n.display_name,
            os: n.os,
            arch: n.arch,
            version: n.version,
            last_seen: None, // Would come from presence tracking
            rx_bytes: 0,
            tx_bytes: 0,
            tags: n.tags,
        })
        .collect();

    Ok(HttpResponse::Ok().json(NodeListResponse {
        total: node_infos.len(),
        nodes: node_infos,
    }))
}

#[get("/nodes/{node_id}")]
pub async fn get_node(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder> {
    let node_id =
        NodeId::from_str(&path).map_err(|_| CoordinatorError::Auth("Invalid node ID".into()))?;

    let metadata = data
        .storage
        .get_node_metadata(&node_id)
        .await?
        .ok_or(CoordinatorError::Auth("Node not found".into()))?;

    Ok(HttpResponse::Ok().json(NodeInfo {
        node_id: metadata.node_id,
        display_name: metadata.display_name,
        os: metadata.os,
        arch: metadata.arch,
        version: metadata.version,
        last_seen: None,
        rx_bytes: 0,
        tx_bytes: 0,
        tags: metadata.tags,
    }))
}

#[delete("/nodes/{node_id}")]
pub async fn delete_node(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder> {
    let node_id =
        NodeId::from_str(&path).map_err(|_| CoordinatorError::Auth("Invalid node ID".into()))?;

    data.storage.delete_node_metadata(&node_id).await?;

    Ok(HttpResponse::NoContent().finish())
}

pub fn configure_nodes_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list_nodes)
        .service(get_node)
        .service(delete_node);
}
