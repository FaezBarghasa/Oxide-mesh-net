---
okf_version: "0.2"
type: Function
title: with_queues
description: Set number of queues
resource: crates/oxide-tun/src/config.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-tun"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-tun/src/config/with_queues
language: rust
---

# with_queues

Set number of queues

## Signature

```rust
impl TunConfig { pub fn with_queues(mut self, num_queues: usize) -> Result<Self> }
```

## Visibility

- `pub`

## Docstring

Set number of queues

## Source
Lines 58–66 in `crates/oxide-tun/src/config.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [config](/crates/oxide-tun/src/config.md) |
