---
okf_version: "0.2"
type: Class
title: TransportConfig
description: Transport configuration
resource: crates/oxide-transport/src/config.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-transport/src/config/TransportConfig
language: rust
---

# TransportConfig

Transport configuration

## Signature

```rust
pub struct TransportConfig
```

## Decorators

- `derive(Debug)`

## Visibility

- `pub`

## Docstring

Transport configuration
[derive(Debug)]

## Methods

- `bind_addrs`
- `max_connections`
- `idle_timeout`
- `keepalive_interval`
- `max_datagram_size`
- `enable_0rtt`
- `congestion_control`
- `server_cert`
- `server_key`
- `root_certs`
- `alpn`

## Source
Lines 12–35 in `crates/oxide-transport/src/config.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [config](/crates/oxide-transport/src/config.md) |
