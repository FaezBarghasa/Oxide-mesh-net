---
okf_version: "0.2"
type: Function
title: set_ipv4
resource: crates/oxide-tun/src/platform/windows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-tun"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-tun/src/platform/windows/set_ipv4_1
language: rust
---

# set_ipv4

## Signature

```rust
fn set_ipv4(&self, addr: std::net::Ipv4Addr, prefix_len: u8) -> Result<()>
```

## Source
Lines 77–80 in `crates/oxide-tun/src/platform/windows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [windows](/crates/oxide-tun/src/platform/windows.md) |
