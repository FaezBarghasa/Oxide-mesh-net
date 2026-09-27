---
okf_version: "0.2"
type: Function
title: set_ipv4
resource: crates/oxide-tun/src/platform/linux.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-tun"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-tun/src/platform/linux/set_ipv4
language: rust
---

# set_ipv4

## Signature

```rust
impl LinuxTunDevice { fn set_ipv4(&self, addr: std::net::Ipv4Addr, prefix_len: u8) -> Result<()> }
```

## Source
Lines 228–281 in `crates/oxide-tun/src/platform/linux.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [linux](/crates/oxide-tun/src/platform/linux.md) |
| calls | [set_ifr_name](/crates/oxide-tun/src/platform/linux/set_ifr_name.md) |
