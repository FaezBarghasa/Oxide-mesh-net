---
okf_version: "0.2"
type: Function
title: extract_node_id
description: Extract node ID from request
resource: crates/oxide-coordinator/src/auth.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/auth/extract_node_id_1
language: rust
---

# extract_node_id

Extract node ID from request

## Signature

```rust
pub fn extract_node_id(&self, req: &actix_web::HttpRequest) -> Result<NodeId>
```

## Visibility

- `pub`

## Docstring

Extract node ID from request

## Source
Lines 209–229 in `crates/oxide-coordinator/src/auth.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [auth](/crates/oxide-coordinator/src/auth.md) |
