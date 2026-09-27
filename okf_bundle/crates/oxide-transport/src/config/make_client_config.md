---
okf_version: "0.2"
type: Function
title: make_client_config
description: Build client configuration
resource: crates/oxide-transport/src/config.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-transport/src/config/make_client_config
language: rust
---

# make_client_config

Build client configuration

## Signature

```rust
pub fn make_client_config(config: &TransportConfig) -> Result<ClientConfig, TransportError>
```

## Visibility

- `pub`

## Docstring

Build client configuration

## Source
Lines 92–120 in `crates/oxide-transport/src/config.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [config](/crates/oxide-transport/src/config.md) |
| called_by | [start](/crates/oxide-transport/src/engine/start.md) |
