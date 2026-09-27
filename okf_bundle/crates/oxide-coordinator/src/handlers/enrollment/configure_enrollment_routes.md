---
okf_version: "0.2"
type: Function
title: configure_enrollment_routes
resource: crates/oxide-coordinator/src/handlers/enrollment.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/handlers/enrollment/configure_enrollment_routes
language: rust
---

# configure_enrollment_routes

## Signature

```rust
pub fn configure_enrollment_routes(cfg: &mut web::ServiceConfig)
```

## Visibility

- `pub`

## Source
Lines 147–151 in `crates/oxide-coordinator/src/handlers/enrollment.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [enrollment](/crates/oxide-coordinator/src/handlers/enrollment.md) |
| called_by | [configure_routes](/crates/oxide-coordinator/src/handlers/mod/configure_routes.md) |
