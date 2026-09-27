---
okf_version: "0.2"
type: Function
title: process_tun_packet
description: Process an inbound/outbound IP packet through data plane pipeline
resource: crates/oxide-daemon/src/engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:33:11Z"
concept_id: crates/oxide-daemon/src/engine/process_tun_packet_1
language: rust
---

# process_tun_packet

Process an inbound/outbound IP packet through data plane pipeline

## Signature

```rust
pub fn process_tun_packet(
        &self,
        packet: &mut [u8],
        l1_cache: &mut L1DirectMappedCache,
    ) -> Option<RouteTarget>
```

## Visibility

- `pub`

## Docstring

Process an inbound/outbound IP packet through data plane pipeline

## Source
Lines 71–123 in `crates/oxide-daemon/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-daemon/src/engine.md) |
| calls | [clamp_tcp_mss](/crates/oxide-tun/src/mss/clamp_tcp_mss.md) |
