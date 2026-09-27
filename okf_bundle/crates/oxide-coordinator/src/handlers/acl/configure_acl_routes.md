---
okf_version: "0.2"
type: Function
title: configure_acl_routes
resource: crates/oxide-coordinator/src/handlers/acl.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/handlers/acl/configure_acl_routes
language: rust
---

# configure_acl_routes

## Signature

```rust
pub fn configure_acl_routes(cfg: &mut web::ServiceConfig)
```

## Visibility

- `pub`

## Source
Lines 138–140 in `crates/oxide-coordinator/src/handlers/acl.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [acl](/crates/oxide-coordinator/src/handlers/acl.md) |
| called_by | [configure_routes](/crates/oxide-coordinator/src/handlers/mod/configure_routes.md) |
