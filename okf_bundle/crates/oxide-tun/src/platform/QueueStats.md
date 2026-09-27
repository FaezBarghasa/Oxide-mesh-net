---
okf_version: "0.2"
type: Class
title: QueueStats
description: Queue statistics
resource: crates/oxide-tun/src/platform.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-tun"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-tun/src/platform/QueueStats
language: rust
---

# QueueStats

Queue statistics

## Signature

```rust
pub struct QueueStats
```

## Decorators

- `derive(Debug, Clone, Default)`

## Visibility

- `pub`

## Docstring

Queue statistics
[derive(Debug, Clone, Default)]

## Methods

- `packets_rx`
- `packets_tx`
- `bytes_rx`
- `bytes_tx`
- `errors_rx`
- `errors_tx`

## Source
Lines 62–69 in `crates/oxide-tun/src/platform.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [platform](/crates/oxide-tun/src/platform.md) |
