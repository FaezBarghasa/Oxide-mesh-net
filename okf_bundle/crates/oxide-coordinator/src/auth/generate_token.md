---
okf_version: "0.2"
type: Function
title: generate_token
description: Generate a JWT token for a node
resource: crates/oxide-coordinator/src/auth.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/auth/generate_token
language: rust
---

# generate_token

Generate a JWT token for a node

## Signature

```rust
impl AuthService { pub fn generate_token(
        &self,
        node_id: NodeId,
        mesh_name: &MeshName,
        capabilities: Vec<String>,
    ) -> Result<String> }
```

## Visibility

- `pub`

## Docstring

Generate a JWT token for a node

## Source
Lines 77–99 in `crates/oxide-coordinator/src/auth.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [auth](/crates/oxide-coordinator/src/auth.md) |
