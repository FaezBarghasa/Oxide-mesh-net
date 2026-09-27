---
okf_version: "0.2"
type: Class
title: ConnectionStats
description: Per-connection statistics
resource: crates/oxide-transport/src/engine.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-transport/src/engine/ConnectionStats
language: rust
---

# ConnectionStats

Per-connection statistics

## Signature

```rust
struct ConnectionStats
```

## Decorators

- `derive(Default)`

## Docstring

Per-connection statistics
[derive(Default)]

## Methods

- `datagrams_sent`
- `datagrams_received`
- `bytes_sent`
- `bytes_received`

## Source
Lines 112–117 in `crates/oxide-transport/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-transport/src/engine.md) |
