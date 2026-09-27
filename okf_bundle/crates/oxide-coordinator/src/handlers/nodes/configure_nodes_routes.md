---
okf_version: "0.2"
type: Function
title: configure_nodes_routes
resource: crates/oxide-coordinator/src/handlers/nodes.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/handlers/nodes/configure_nodes_routes
language: rust
---

# configure_nodes_routes

## Signature

```rust
pub fn configure_nodes_routes(cfg: &mut web::ServiceConfig)
```

## Visibility

- `pub`

## Source
Lines 93–97 in `crates/oxide-coordinator/src/handlers/nodes.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [nodes](/crates/oxide-coordinator/src/handlers/nodes.md) |
| called_by | [configure_routes](/crates/oxide-coordinator/src/handlers/mod/configure_routes.md) |
