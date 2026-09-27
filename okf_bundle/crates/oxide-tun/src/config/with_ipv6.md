---
okf_version: "0.2"
type: Function
title: with_ipv6
description: Set IPv6 address
resource: crates/oxide-tun/src/config.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-tun"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-tun/src/config/with_ipv6
language: rust
---

# with_ipv6

Set IPv6 address

## Signature

```rust
impl TunConfig { pub fn with_ipv6(mut self, ipv6: IpNet) -> Result<Self> }
```

## Visibility

- `pub`

## Docstring

Set IPv6 address

## Source
Lines 91–99 in `crates/oxide-tun/src/config.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [config](/crates/oxide-tun/src/config.md) |
