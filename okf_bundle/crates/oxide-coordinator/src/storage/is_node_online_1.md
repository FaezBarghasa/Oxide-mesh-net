---
okf_version: "0.2"
type: Function
title: is_node_online
resource: crates/oxide-coordinator/src/storage.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/storage/is_node_online_1
language: rust
---

# is_node_online

## Signature

```rust
impl Storage { pub fn is_node_online(&self, node_id: &NodeId) -> Result<bool> }
```

## Visibility

- `pub`

## Source
Lines 193–195 in `crates/oxide-coordinator/src/storage.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [storage](/crates/oxide-coordinator/src/storage.md) |
