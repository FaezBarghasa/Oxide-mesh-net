---
okf_version: "0.2"
type: Class
title: RaftConfig
description: Raft consensus configuration
resource: crates/oxide-coordinator/src/config.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/config/RaftConfig
language: rust
---

# RaftConfig

Raft consensus configuration

## Signature

```rust
pub struct RaftConfig
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Raft consensus configuration
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `node_id`
- `peers`
- `election_timeout`
- `heartbeat_interval`

## Source
Lines 163–172 in `crates/oxide-coordinator/src/config.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [config](/crates/oxide-coordinator/src/config.md) |
