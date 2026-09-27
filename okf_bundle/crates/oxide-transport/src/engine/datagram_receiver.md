---
okf_version: "0.2"
type: Function
title: datagram_receiver
description: Datagram receiver loop
resource: crates/oxide-transport/src/engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-transport/src/engine/datagram_receiver
language: rust
---

# datagram_receiver

Datagram receiver loop

## Signature

```rust
impl TransportEngine { fn datagram_receiver(
        connections: Arc<RwLock<HashMap<NodeId, ConnectionState>>>,
        stats: Arc<RwLock<TransportStats>>,
        event_tx: broadcast::Sender<TransportEvent>,
        node_id: NodeId,
    ) }
```

## Docstring

Datagram receiver loop

## Source
Lines 471–524 in `crates/oxide-transport/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-transport/src/engine.md) |
