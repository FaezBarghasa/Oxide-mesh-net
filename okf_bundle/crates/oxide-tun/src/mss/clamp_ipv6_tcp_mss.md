---
okf_version: "0.2"
type: Function
title: clamp_ipv6_tcp_mss
resource: crates/oxide-tun/src/mss.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-tun"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
concept_id: crates/oxide-tun/src/mss/clamp_ipv6_tcp_mss
language: rust
---

# clamp_ipv6_tcp_mss

## Signature

```rust
fn clamp_ipv6_tcp_mss(packet: &mut [u8], max_mss: u16) -> Result<bool>
```

## Source
Lines 76–122 in `crates/oxide-tun/src/mss.rs`
