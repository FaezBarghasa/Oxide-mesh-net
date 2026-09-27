---
okf_version: "0.2"
type: Function
title: configure_tunnel_routes
description: Configure tunnel routes
resource: crates/oxide-coordinator/src/handlers/tunnel_ws.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:33:11Z"
concept_id: crates/oxide-coordinator/src/handlers/tunnel_ws/configure_tunnel_routes
language: rust
---

# configure_tunnel_routes

Configure tunnel routes

## Signature

```rust
pub fn configure_tunnel_routes(cfg: &mut web::ServiceConfig)
```

## Visibility

- `pub`

## Docstring

Configure tunnel routes

## Source
Lines 84–86 in `crates/oxide-coordinator/src/handlers/tunnel_ws.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tunnel_ws](/crates/oxide-coordinator/src/handlers/tunnel_ws.md) |
| called_by | [configure_routes](/crates/oxide-coordinator/src/handlers/mod/configure_routes.md) |
