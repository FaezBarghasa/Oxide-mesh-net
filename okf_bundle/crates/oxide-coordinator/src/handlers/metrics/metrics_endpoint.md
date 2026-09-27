---
okf_version: "0.2"
type: Function
title: metrics_endpoint
resource: crates/oxide-coordinator/src/handlers/metrics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/handlers/metrics/metrics_endpoint
language: rust
---

# metrics_endpoint

## Signature

```rust
pub fn metrics_endpoint(metrics: web::Data<PrometheusMetrics>) -> impl Responder
```

## Visibility

- `pub`

## Source
Lines 7–16 in `crates/oxide-coordinator/src/handlers/metrics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [metrics](/crates/oxide-coordinator/src/handlers/metrics.md) |
