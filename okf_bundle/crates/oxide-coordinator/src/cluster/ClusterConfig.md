---
okf_version: "0.2"
type: Class
title: ClusterConfig
description: Configuration for Coordinator Clustering
resource: crates/oxide-coordinator/src/cluster.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/cluster/ClusterConfig
language: rust
---

# ClusterConfig

Configuration for Coordinator Clustering

## Signature

```rust
pub struct ClusterConfig
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Configuration for Coordinator Clustering
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `node_id`
- `cluster_peers`
- `heartbeat_interval`
- `election_timeout_min`
- `election_timeout_max`

## Source
Lines 43–49 in `crates/oxide-coordinator/src/cluster.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cluster](/crates/oxide-coordinator/src/cluster.md) |
