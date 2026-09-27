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
