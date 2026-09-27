---
okf_version: "0.2"
type: Class
title: DaemonConfig
description: Comprehensive configuration for oxide-daemon
resource: crates/oxide-daemon/src/config.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:27:46Z"
concept_id: crates/oxide-daemon/src/config/DaemonConfig
language: rust
---

# DaemonConfig

Comprehensive configuration for oxide-daemon

## Signature

```rust
pub struct DaemonConfig
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Comprehensive configuration for oxide-daemon
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `node_id`
- `mesh_name`
- `coordinator_url`
- `tun_name`
- `mtu`
- `listen_port`
- `enable_dns`
- `enable_mss_clamp`
- `socket_path`
- `static_routes`

## Source
Lines 17–29 in `crates/oxide-daemon/src/config.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [config](/crates/oxide-daemon/src/config.md) |
