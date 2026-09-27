---
okf_version: "0.2"
type: Function
title: set_ifr_name
resource: crates/oxide-tun/src/platform/linux.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-tun"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-tun/src/platform/linux/set_ifr_name
language: rust
---

# set_ifr_name

## Signature

```rust
fn set_ifr_name(ifr: &mut IfReq, name: &str)
```

## Source
Lines 48–55 in `crates/oxide-tun/src/platform/linux.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [linux](/crates/oxide-tun/src/platform/linux.md) |
| called_by | [create_tun_fd](/crates/oxide-tun/src/platform/linux/create_tun_fd.md) |
| called_by | [set_ipv4](/crates/oxide-tun/src/platform/linux/set_ipv4.md) |
| called_by | [set_mtu](/crates/oxide-tun/src/platform/linux/set_mtu.md) |
| called_by | [set_up](/crates/oxide-tun/src/platform/linux/set_up.md) |
