---
okf_version: "0.2"
type: Function
title: list_nodes
description: "[get(\"/nodes\")]"
resource: crates/oxide-coordinator/src/handlers/nodes.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/handlers/nodes/list_nodes
language: rust
---

# list_nodes

[get("/nodes")]

## Signature

```rust
pub fn list_nodes(data: web::Data<AppState>) -> Result<impl Responder>
```

## Decorators

- `get("/nodes")`

## Visibility

- `pub`

## Docstring

[get("/nodes")]

## Source
Lines 30–51 in `crates/oxide-coordinator/src/handlers/nodes.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [nodes](/crates/oxide-coordinator/src/handlers/nodes.md) |
