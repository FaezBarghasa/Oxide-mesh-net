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
timestamp: "2026-09-27T06:44:41Z"
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

## Relationships

| Type | Target |
|------|--------|
| related | [mss](/crates/oxide-tun/src/mss.md) |
| calls | [clamp_tcp_segment](/crates/oxide-tun/src/mss/clamp_tcp_segment.md) |
| called_by | [clamp_tcp_mss](/crates/oxide-tun/src/mss/clamp_tcp_mss.md) |
