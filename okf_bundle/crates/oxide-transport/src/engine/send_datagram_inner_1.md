---
okf_version: "0.2"
type: Function
title: send_datagram_inner
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
concept_id: crates/oxide-transport/src/engine/send_datagram_inner_1
language: rust
---

# send_datagram_inner

Send a datagram

## Signature

```rust
fn send_datagram_inner(
        connections: &Arc<RwLock<HashMap<NodeId, ConnectionState>>>,
        stats: &Arc<RwLock<TransportStats>>,
        node_id: NodeId,
        packet: WirePacket,
    ) -> Result<()>
```

## Docstring

Send a datagram

## Source
Lines 423–451 in `crates/oxide-transport/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-transport/src/engine.md) |
