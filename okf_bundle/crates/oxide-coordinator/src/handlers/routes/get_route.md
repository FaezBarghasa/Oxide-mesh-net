---
okf_version: "0.2"
type: Function
title: get_route
description: "[get(\"/routes/{prefix}\")]"
resource: crates/oxide-coordinator/src/handlers/routes.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/handlers/routes/get_route
language: rust
---

# get_route

[get("/routes/{prefix}")]

## Signature

```rust
pub fn get_route(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder>
```

## Decorators

- `get("/routes/{prefix}")`

## Visibility

- `pub`

## Docstring

[get("/routes/{prefix}")]

## Source
Lines 48–69 in `crates/oxide-coordinator/src/handlers/routes.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [routes](/crates/oxide-coordinator/src/handlers/routes.md) |
