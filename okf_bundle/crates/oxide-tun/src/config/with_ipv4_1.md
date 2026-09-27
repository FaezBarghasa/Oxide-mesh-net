---
okf_version: "0.2"
type: Function
title: with_ipv4
description: Set IPv4 address
resource: crates/oxide-tun/src/config.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-tun"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-tun/src/config/with_ipv4_1
language: rust
---

# with_ipv4

Set IPv4 address

## Signature

```rust
pub fn with_ipv4(mut self, ipv4: IpNet) -> Result<Self>
```

## Visibility

- `pub`

## Docstring

Set IPv4 address

## Source
Lines 80–88 in `crates/oxide-tun/src/config.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [config](/crates/oxide-tun/src/config.md) |
