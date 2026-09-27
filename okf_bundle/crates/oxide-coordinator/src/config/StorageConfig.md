---
okf_version: "0.2"
type: Class
title: StorageConfig
description: Storage configuration
resource: crates/oxide-coordinator/src/config.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/config/StorageConfig
language: rust
---

# StorageConfig

Storage configuration

## Signature

```rust
pub struct StorageConfig
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Storage configuration
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `backend`
- `data_dir`
- `surreal_url`
- `surreal_ns`
- `surreal_db`
- `surreal_user`
- `surreal_pass`
- `raft`

## Source
Lines 117–134 in `crates/oxide-coordinator/src/config.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [config](/crates/oxide-coordinator/src/config.md) |
