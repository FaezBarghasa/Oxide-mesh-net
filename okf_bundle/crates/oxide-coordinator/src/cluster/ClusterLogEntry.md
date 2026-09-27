---
okf_version: "0.2"
type: Class
title: ClusterLogEntry
description: Log mutation entry replicated across the cluster
resource: crates/oxide-coordinator/src/cluster.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/cluster/ClusterLogEntry
language: rust
---

# ClusterLogEntry

Log mutation entry replicated across the cluster

## Signature

```rust
pub enum ClusterLogEntry
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Log mutation entry replicated across the cluster
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `user_code`
- `node_id`
- `approved_at`

## Source
Lines 28–39 in `crates/oxide-coordinator/src/cluster.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cluster](/crates/oxide-coordinator/src/cluster.md) |
