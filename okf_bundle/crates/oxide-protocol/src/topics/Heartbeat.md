---
okf_version: "0.2"
type: Class
title: Heartbeat
description: Heartbeat payload
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/Heartbeat
language: rust
---

# Heartbeat

Heartbeat payload

## Signature

```rust
pub struct Heartbeat
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Heartbeat payload
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `node_id`
- `timestamp`
- `active_peers`
- `rx_bytes`
- `tx_bytes`
- `cpu_usage`
- `mem_usage`

## Source
Lines 283–291 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
