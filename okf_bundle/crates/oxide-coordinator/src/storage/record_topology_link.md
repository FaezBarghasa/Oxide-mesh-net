---
okf_version: "0.2"
type: Function
title: record_topology_link
resource: crates/oxide-coordinator/src/storage.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/storage/record_topology_link
language: rust
---

# record_topology_link

## Signature

```rust
fn record_topology_link(
        &self,
        _from: &NodeId,
        _to: &NodeId,
        _latency_ms: f32,
        _loss_rate: f32,
    ) -> Result<()>
```

## Source
Lines 47–55 in `crates/oxide-coordinator/src/storage.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [storage](/crates/oxide-coordinator/src/storage.md) |
