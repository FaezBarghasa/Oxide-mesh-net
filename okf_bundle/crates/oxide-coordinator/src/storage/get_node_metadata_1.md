---
okf_version: "0.2"
type: Function
title: get_node_metadata
resource: crates/oxide-coordinator/src/storage.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/storage/get_node_metadata_1
language: rust
---

# get_node_metadata

## Signature

```rust
pub fn get_node_metadata(&self, node_id: &NodeId) -> Result<Option<NodeMetadata>>
```

## Visibility

- `pub`

## Source
Lines 149–151 in `crates/oxide-coordinator/src/storage.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [storage](/crates/oxide-coordinator/src/storage.md) |
