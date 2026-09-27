pub mod acl;
pub mod enrollment;
pub mod health;
pub mod metrics;
pub mod nodes;
pub mod routes;
pub mod tunnel_ws;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    enrollment::configure_enrollment_routes(cfg);
    nodes::configure_nodes_routes(cfg);
    acl::configure_acl_routes(cfg);
    routes::configure_routes_routes(cfg);
    tunnel_ws::configure_tunnel_routes(cfg);
}
