//! Route management handlers

use actix_web::{web, HttpResponse, Responder, get, delete};
use serde::Serialize;
use crate::{AppState, error::{CoordinatorError, Result}};
use oxide_core::{NodeId, OverlayPrefix};

#[derive(Serialize)]
pub struct RouteListResponse {
    pub routes: Vec<RouteInfo>,
    pub total: usize,
}

#[derive(Serialize)]
pub struct RouteInfo {
    pub prefix: OverlayPrefix,
    pub node_id: NodeId,
    pub metric: u32,
    pub next_hop: Option<String>,
    pub communities: Vec<String>,
    pub advertised_at: i64,
}

#[get("/routes")]
pub async fn list_routes(data: web::Data<AppState>) -> Result<impl Responder> {
    let routes = data.storage.list_routes().await?;
    let route_infos: Vec<RouteInfo> = routes.into_iter().map(|r| RouteInfo {
        prefix: r.prefix,
        node_id: r.node_id,
        metric: r.metric,
        next_hop: r.next_hop.map(|ip| ip.to_string()),
        communities: r.communities,
        advertised_at: r.timestamp,
    }).collect();

    Ok(HttpResponse::Ok().json(RouteListResponse {
        total: route_infos.len(),
        routes: route_infos,
    }))
}

#[get("/routes/{prefix}")]
pub async fn get_route(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder> {
    let prefix = OverlayPrefix::from_str(&path)
        .map_err(|_| CoordinatorError::Config("Invalid prefix".into()))?;

    let route = data.storage.get_route(&prefix).await?
        .ok_or(CoordinatorError::Auth("Route not found".into()))?;

    Ok(HttpResponse::Ok().json(RouteInfo {
        prefix: route.prefix,
        node_id: route.node_id,
        metric: route.metric,
        next_hop: route.next_hop.map(|ip| ip.to_string()),
        communities: route.communities,
        advertised_at: route.timestamp,
    }))
}

#[delete("/routes/{prefix}")]
pub async fn withdraw_route(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder> {
    let prefix = OverlayPrefix::from_str(&path)
        .map_err(|_| CoordinatorError::Config("Invalid prefix".into()))?;

    data.storage.delete_route(&prefix).await?;

    // Publish withdrawal to MQTT
    // data.broker.publish(...).await?;

    Ok(HttpResponse::NoContent().finish())
}

pub fn configure_routes_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list_routes)
       .service(get_route)
       .service(withdraw_route);
}