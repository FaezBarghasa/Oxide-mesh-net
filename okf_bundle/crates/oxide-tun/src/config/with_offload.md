---
okf_version: "0.2"
type: Function
title: with_offload
description: Enable/disable offload
resource: crates/oxide-tun/src/config.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-tun"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-tun/src/config/with_offload
language: rust
---

# with_offload

Enable/disable offload

## Signature

```rust
impl TunConfig { pub fn with_offload(mut self, offload: bool) -> Self }
```

## Visibility

- `pub`

## Docstring

Enable/disable offload

## Source
Lines 108–111 in `crates/oxide-tun/src/config.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [config](/crates/oxide-tun/src/config.md) |
