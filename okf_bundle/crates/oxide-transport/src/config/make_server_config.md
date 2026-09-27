---
okf_version: "0.2"
type: Function
title: make_server_config
description: Build server configuration
resource: crates/oxide-transport/src/config.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-transport/src/config/make_server_config
language: rust
---

# make_server_config

Build server configuration

## Signature

```rust
pub fn make_server_config(config: &TransportConfig) -> Result<ServerConfig, TransportError>
```

## Visibility

- `pub`

## Docstring

Build server configuration

## Source
Lines 123–156 in `crates/oxide-transport/src/config.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [config](/crates/oxide-transport/src/config.md) |
| called_by | [start](/crates/oxide-transport/src/engine/start.md) |
