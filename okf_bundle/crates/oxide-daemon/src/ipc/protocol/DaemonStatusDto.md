---
okf_version: "0.2"
type: Class
title: DaemonStatusDto
description: High-level daemon status DTO
resource: crates/oxide-daemon/src/ipc/protocol.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:33:11Z"
concept_id: crates/oxide-daemon/src/ipc/protocol/DaemonStatusDto
language: rust
---

# DaemonStatusDto

High-level daemon status DTO

## Signature

```rust
pub struct DaemonStatusDto
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize, PartialEq)`

## Visibility

- `pub`

## Docstring

High-level daemon status DTO
[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

## Methods

- `node_id`
- `tun_name`
- `mtu`
- `active_peers`
- `confirmed_pmtu`
- `circuit_breaker_state`
- `port_hopping_epoch`
- `current_port`
- `dns_active`
- `mss_clamping_active`
- `uptime_secs`

## Source
Lines 35–47 in `crates/oxide-daemon/src/ipc/protocol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [protocol](/crates/oxide-daemon/src/ipc/protocol.md) |
