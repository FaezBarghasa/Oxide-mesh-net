---
okf_version: "0.2"
type: Function
title: promote_to_leader
description: Promote to cluster leader upon election win
resource: crates/oxide-coordinator/src/cluster.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/cluster/promote_to_leader
language: rust
---

# promote_to_leader

Promote to cluster leader upon election win

## Signature

```rust
impl ClusterEngine { pub fn promote_to_leader(&self) }
```

## Visibility

- `pub`

## Docstring

Promote to cluster leader upon election win

## Source
Lines 140–148 in `crates/oxide-coordinator/src/cluster.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cluster](/crates/oxide-coordinator/src/cluster.md) |
