---
okf_version: "0.2"
type: Function
title: withdraw_route
description: "[delete(\"/routes/{prefix}\")]"
resource: crates/oxide-coordinator/src/handlers/routes.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/handlers/routes/withdraw_route
language: rust
---

# withdraw_route

[delete("/routes/{prefix}")]

## Signature

```rust
pub fn withdraw_route(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder>
```

## Decorators

- `delete("/routes/{prefix}")`

## Visibility

- `pub`

## Docstring

[delete("/routes/{prefix}")]

## Source
Lines 72–85 in `crates/oxide-coordinator/src/handlers/routes.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [routes](/crates/oxide-coordinator/src/handlers/routes.md) |
