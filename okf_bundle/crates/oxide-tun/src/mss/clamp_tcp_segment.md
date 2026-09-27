---
okf_version: "0.2"
type: Function
title: clamp_tcp_segment
resource: crates/oxide-tun/src/mss.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-tun"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:44:41Z"
concept_id: crates/oxide-tun/src/mss/clamp_tcp_segment
language: rust
---

# clamp_tcp_segment

## Signature

```rust
fn clamp_tcp_segment(tcp: &mut [u8], max_mss: u16) -> Result<bool>
```

## Source
Lines 124–209 in `crates/oxide-tun/src/mss.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mss](/crates/oxide-tun/src/mss.md) |
| calls | [update_checksum_16](/crates/oxide-tun/src/mss/update_checksum_16.md) |
| called_by | [clamp_ipv4_tcp_mss](/crates/oxide-tun/src/mss/clamp_ipv4_tcp_mss.md) |
| called_by | [clamp_ipv6_tcp_mss](/crates/oxide-tun/src/mss/clamp_ipv6_tcp_mss.md) |
