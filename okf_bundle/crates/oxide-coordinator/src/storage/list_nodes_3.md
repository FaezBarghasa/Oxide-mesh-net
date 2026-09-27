---
okf_version: "0.2"
type: Function
title: list_nodes
resource: crates/oxide-coordinator/src/storage.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/storage/list_nodes_3
language: rust
---

# list_nodes

## Signature

```rust
fn list_nodes(&self) -> Result<Vec<NodeMetadata>>
```

## Source
Lines 449–485 in `crates/oxide-coordinator/src/storage.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [storage](/crates/oxide-coordinator/src/storage.md) |
| calls | [Storage](/crates/oxide-coordinator/src/storage/Storage.md) |
