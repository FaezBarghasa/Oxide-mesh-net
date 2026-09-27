---
okf_version: "0.2"
type: Class
title: CoordinatorConfig
description: Coordinator configuration
resource: crates/oxide-coordinator/src/config.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/config/CoordinatorConfig
language: rust
---

# CoordinatorConfig

Coordinator configuration

## Signature

```rust
pub struct CoordinatorConfig
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Coordinator configuration
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `mesh_name`
- `mqtt_listeners`
- `http`
- `storage`
- `auth`
- `cluster`
- `rate_limit`
- `tls`

## Source
Lines 10–27 in `crates/oxide-coordinator/src/config.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [config](/crates/oxide-coordinator/src/config.md) |
