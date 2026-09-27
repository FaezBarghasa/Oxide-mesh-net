---
okf_version: "0.2"
type: Function
title: record_presence
description: "Extended capabilities (presence & graph)"
resource: crates/oxide-coordinator/src/storage.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/storage/record_presence
language: rust
---

# record_presence

Extended capabilities (presence & graph)

## Signature

```rust
fn record_presence(&self, _node_id: &NodeId, _ttl: Duration) -> Result<()>
```

## Docstring

Extended capabilities (presence & graph)

## Source
Lines 39–41 in `crates/oxide-coordinator/src/storage.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [storage](/crates/oxide-coordinator/src/storage.md) |
