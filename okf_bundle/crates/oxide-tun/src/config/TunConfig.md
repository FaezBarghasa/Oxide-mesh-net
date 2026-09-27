---
okf_version: "0.2"
type: Class
title: TunConfig
description: TUN interface configuration
resource: crates/oxide-tun/src/config.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-tun"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-tun/src/config/TunConfig
language: rust
---

# TunConfig

TUN interface configuration

## Signature

```rust
pub struct TunConfig
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

TUN interface configuration
[derive(Debug, Clone)]

## Methods

- `name`
- `mtu`
- `num_queues`
- `ipv4`
- `ipv6`
- `up`
- `offload`
- `persistent`
- `owner_uid`
- `owner_gid`

## Source
Lines 8–29 in `crates/oxide-tun/src/config.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [config](/crates/oxide-tun/src/config.md) |
