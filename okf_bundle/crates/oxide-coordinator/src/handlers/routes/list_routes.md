---
okf_version: "0.2"
type: Function
title: list_routes
description: "[get(\"/routes\")]"
resource: crates/oxide-coordinator/src/handlers/routes.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/handlers/routes/list_routes
language: rust
---

# list_routes

[get("/routes")]

## Signature

```rust
pub fn list_routes(data: web::Data<AppState>) -> Result<impl Responder>
```

## Decorators

- `get("/routes")`

## Visibility

- `pub`

## Docstring

[get("/routes")]

## Source
Lines 27–45 in `crates/oxide-coordinator/src/handlers/routes.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [routes](/crates/oxide-coordinator/src/handlers/routes.md) |
