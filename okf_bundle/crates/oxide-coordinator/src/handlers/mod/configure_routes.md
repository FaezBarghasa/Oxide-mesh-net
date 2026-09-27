---
okf_version: "0.2"
type: Function
title: configure_routes
resource: crates/oxide-coordinator/src/handlers/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:24:21Z"
concept_id: crates/oxide-coordinator/src/handlers/mod/configure_routes
language: rust
---

# configure_routes

## Signature

```rust
pub fn configure_routes(cfg: &mut web::ServiceConfig)
```

## Visibility

- `pub`

## Source
Lines 11–17 in `crates/oxide-coordinator/src/handlers/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handlers](/crates/oxide-coordinator/src/handlers/mod.md) |
| calls | [configure_enrollment_routes](/crates/oxide-coordinator/src/handlers/enrollment/configure_enrollment_routes.md) |
| calls | [configure_nodes_routes](/crates/oxide-coordinator/src/handlers/nodes/configure_nodes_routes.md) |
| calls | [configure_acl_routes](/crates/oxide-coordinator/src/handlers/acl/configure_acl_routes.md) |
| calls | [configure_routes_routes](/crates/oxide-coordinator/src/handlers/routes/configure_routes_routes.md) |
| calls | [configure_tunnel_routes](/crates/oxide-coordinator/src/handlers/tunnel_ws/configure_tunnel_routes.md) |
