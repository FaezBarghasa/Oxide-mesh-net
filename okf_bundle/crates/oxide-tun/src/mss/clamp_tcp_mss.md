---
okf_version: "0.2"
type: Function
title: clamp_tcp_mss
description: Dynamic TCP MSS Clamper
resource: crates/oxide-tun/src/mss.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-tun"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:44:41Z"
concept_id: crates/oxide-tun/src/mss/clamp_tcp_mss
language: rust
---

# clamp_tcp_mss

Dynamic TCP MSS Clamper

## Signature

```rust
pub fn clamp_tcp_mss(packet: &mut [u8], max_mss: u16) -> Result<bool>
```

## Visibility

- `pub`

## Docstring

Dynamic TCP MSS Clamper

Checks if an IPv4 or IPv6 packet is a TCP SYN or SYN-ACK with an MSS option exceeding `max_mss`.
If so, rewrites the MSS option in-place and updates the TCP checksum incrementally.

Returns `Ok(true)` if clamped, `Ok(false)` if no clamping was needed or packet was not a TCP SYN.

## Source
Lines 38–49 in `crates/oxide-tun/src/mss.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mss](/crates/oxide-tun/src/mss.md) |
| calls | [clamp_ipv4_tcp_mss](/crates/oxide-tun/src/mss/clamp_ipv4_tcp_mss.md) |
| calls | [clamp_ipv6_tcp_mss](/crates/oxide-tun/src/mss/clamp_ipv6_tcp_mss.md) |
| called_by | [process_tun_packet](/crates/oxide-daemon/src/engine/process_tun_packet.md) |
| called_by | [test_clamp_ipv4_tcp_syn](/crates/oxide-tun/src/mss/test_clamp_ipv4_tcp_syn.md) |
| called_by | [test_clamp_unnecessary_mss](/crates/oxide-tun/src/mss/test_clamp_unnecessary_mss.md) |
