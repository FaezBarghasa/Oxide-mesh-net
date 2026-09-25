//! Metrics handlers

use actix_web::{web, HttpResponse, Responder};
use actix_web_prom::PrometheusMetrics;
use crate::AppState;

pub async fn metrics_endpoint(metrics: web::Data<PrometheusMetrics>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4; charset=utf-8")
        .body(metrics.get_metric_text())
}