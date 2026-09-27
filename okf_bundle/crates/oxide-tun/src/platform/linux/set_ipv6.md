---
okf_version: "0.2"
type: Function
title: set_ipv6
resource: crates/oxide-tun/src/platform/linux.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-tun"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-tun/src/platform/linux/set_ipv6
language: rust
---

# set_ipv6

## Signature

```rust
impl LinuxTunDevice { fn set_ipv6(&self, _addr: std::net::Ipv6Addr, _prefix_len: u8) -> Result<()> }
```

## Source
Lines 283–286 in `crates/oxide-tun/src/platform/linux.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [linux](/crates/oxide-tun/src/platform/linux.md) |
