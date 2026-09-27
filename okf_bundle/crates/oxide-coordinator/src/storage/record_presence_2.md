---
okf_version: "0.2"
type: Function
title: record_presence
resource: crates/oxide-coordinator/src/storage.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/storage/record_presence_2
language: rust
---

# record_presence

## Signature

```rust
pub fn record_presence(&self, node_id: &NodeId, ttl: Duration) -> Result<()>
```

## Visibility

- `pub`

## Source
Lines 189–191 in `crates/oxide-coordinator/src/storage.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [storage](/crates/oxide-coordinator/src/storage.md) |
