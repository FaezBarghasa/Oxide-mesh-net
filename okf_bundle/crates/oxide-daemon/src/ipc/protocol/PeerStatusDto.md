---
okf_version: "0.2"
type: Class
title: PeerStatusDto
description: Peer connection state DTO
resource: crates/oxide-daemon/src/ipc/protocol.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:33:11Z"
concept_id: crates/oxide-daemon/src/ipc/protocol/PeerStatusDto
language: rust
---

# PeerStatusDto

Peer connection state DTO

## Signature

```rust
pub struct PeerStatusDto
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize, PartialEq)`

## Visibility

- `pub`

## Docstring

Peer connection state DTO
[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

## Methods

- `node_id`
- `overlay_ip`
- `endpoint`
- `pmtu`
- `rtt_ms`
- `rx_bytes`
- `tx_bytes`
- `last_seen_secs`

## Source
Lines 60–69 in `crates/oxide-daemon/src/ipc/protocol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [protocol](/crates/oxide-daemon/src/ipc/protocol.md) |
