---
okf_version: "0.2"
type: Function
title: send_datagram
description: Send a datagram
resource: crates/oxide-transport/src/engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-transport/src/engine/send_datagram
language: rust
---

# send_datagram

Send a datagram

## Signature

```rust
impl TransportHandle { pub fn send_datagram(&self, node_id: NodeId, packet: WirePacket) -> Result<()> }
```

## Visibility

- `pub`

## Docstring

Send a datagram

## Source
Lines 693–702 in `crates/oxide-transport/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-transport/src/engine.md) |
