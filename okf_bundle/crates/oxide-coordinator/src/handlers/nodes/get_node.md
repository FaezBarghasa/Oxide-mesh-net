---
okf_version: "0.2"
type: Function
title: get_node
description: "[get(\"/nodes/{node_id}\")]"
resource: crates/oxide-coordinator/src/handlers/nodes.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/handlers/nodes/get_node
language: rust
---

# get_node

[get("/nodes/{node_id}")]

## Signature

```rust
pub fn get_node(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder>
```

## Decorators

- `get("/nodes/{node_id}")`

## Visibility

- `pub`

## Docstring

[get("/nodes/{node_id}")]

## Source
Lines 54–78 in `crates/oxide-coordinator/src/handlers/nodes.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [nodes](/crates/oxide-coordinator/src/handlers/nodes.md) |
