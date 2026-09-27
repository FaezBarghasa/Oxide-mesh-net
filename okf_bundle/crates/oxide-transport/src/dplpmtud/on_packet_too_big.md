---
okf_version: "0.2"
type: Function
title: on_packet_too_big
description: Explicit ICMP Packet-Too-Big notification handler
resource: crates/oxide-transport/src/dplpmtud.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:14:57Z"
concept_id: crates/oxide-transport/src/dplpmtud/on_packet_too_big
language: rust
---

# on_packet_too_big

Explicit ICMP Packet-Too-Big notification handler

## Signature

```rust
impl DplpmtudEngine { pub fn on_packet_too_big(&mut self, advertised_mtu: u16) }
```

## Visibility

- `pub`

## Docstring

Explicit ICMP Packet-Too-Big notification handler

## Source
Lines 192–199 in `crates/oxide-transport/src/dplpmtud.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dplpmtud](/crates/oxide-transport/src/dplpmtud.md) |
