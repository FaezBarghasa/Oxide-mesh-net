---
okf_version: "0.2"
type: Function
title: is_device_grant_authorized
description: Check device grant authorization status across the cluster
resource: crates/oxide-coordinator/src/cluster.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/cluster/is_device_grant_authorized
language: rust
---

# is_device_grant_authorized

Check device grant authorization status across the cluster

## Signature

```rust
impl ClusterEngine { pub fn is_device_grant_authorized(&self, user_code: &str) -> Option<(NodeId, i64)> }
```

## Visibility

- `pub`

## Docstring

Check device grant authorization status across the cluster

## Source
Lines 134–137 in `crates/oxide-coordinator/src/cluster.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cluster](/crates/oxide-coordinator/src/cluster.md) |
