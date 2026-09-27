---
okf_version: "0.2"
type: Class
title: TransportStats
description: Transport statistics
resource: crates/oxide-transport/src/engine.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-transport/src/engine/TransportStats
language: rust
---

# TransportStats

Transport statistics

## Signature

```rust
pub struct TransportStats
```

## Decorators

- `derive(Debug, Clone, Default)`

## Visibility

- `pub`

## Docstring

Transport statistics
[derive(Debug, Clone, Default)]

## Methods

- `active_connections`
- `total_connections`
- `datagrams_sent`
- `datagrams_received`
- `bytes_sent`
- `bytes_received`
- `streams_opened`
- `streams_closed`
- `connection_errors`

## Source
Lines 88–98 in `crates/oxide-transport/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-transport/src/engine.md) |
