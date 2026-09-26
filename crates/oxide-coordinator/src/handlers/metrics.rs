//! Metrics handlers

use actix_web::{web, HttpResponse, Responder};
use actix_web_prometheus::PrometheusMetrics;
use prometheus::{Encoder, TextEncoder};

pub async fn metrics_endpoint(metrics: web::Data<PrometheusMetrics>) -> impl Responder {
    let encoder = TextEncoder::new();
    let mut buffer = Vec::new();
    let metric_families = metrics.registry.gather();
    let _ = encoder.encode(&metric_families[..], &mut buffer);

    HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4; charset=utf-8")
        .body(buffer)
}