---
okf_version: "0.2"
type: Function
title: delete_node_metadata
resource: crates/oxide-coordinator/src/storage.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/storage/delete_node_metadata_7
language: rust
---

# delete_node_metadata

## Signature

```rust
fn delete_node_metadata(&self, node_id: &NodeId) -> Result<()>
```

## Source
Lines 879–884 in `crates/oxide-coordinator/src/storage.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [storage](/crates/oxide-coordinator/src/storage.md) |
| calls | [Storage](/crates/oxide-coordinator/src/storage/Storage.md) |
