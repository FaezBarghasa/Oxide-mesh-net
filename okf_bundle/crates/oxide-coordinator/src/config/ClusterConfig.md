---
okf_version: "0.2"
type: Class
title: ClusterConfig
description: Cluster configuration
resource: crates/oxide-coordinator/src/config.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/config/ClusterConfig
language: rust
---

# ClusterConfig

Cluster configuration

## Signature

```rust
pub struct ClusterConfig
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Cluster configuration
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `name`
- `node_id`
- `peers`
- `gossip_interval`

## Source
Lines 226–235 in `crates/oxide-coordinator/src/config.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [config](/crates/oxide-coordinator/src/config.md) |
