---
okf_version: "0.2"
type: Function
title: delete_node
description: "[delete(\"/nodes/{node_id}\")]"
resource: crates/oxide-coordinator/src/handlers/nodes.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/handlers/nodes/delete_node
language: rust
---

# delete_node

[delete("/nodes/{node_id}")]

## Signature

```rust
pub fn delete_node(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder>
```

## Decorators

- `delete("/nodes/{node_id}")`

## Visibility

- `pub`

## Docstring

[delete("/nodes/{node_id}")]

## Source
Lines 81–91 in `crates/oxide-coordinator/src/handlers/nodes.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [nodes](/crates/oxide-coordinator/src/handlers/nodes.md) |
