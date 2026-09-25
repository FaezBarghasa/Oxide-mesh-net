//! HTTP route handlers

pub mod health;
pub mod metrics;
pub mod enrollment;
pub mod nodes;
pub mod acl;
pub mod routes;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    enrollment::configure_enrollment_routes(cfg);
    nodes::configure_nodes_routes(cfg);
    acl::configure_acl_routes(cfg);
    routes::configure_routes_routes(cfg);
}