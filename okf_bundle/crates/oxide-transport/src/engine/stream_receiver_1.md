---
okf_version: "0.2"
type: Function
title: stream_receiver
description: Stream receiver loop
resource: crates/oxide-transport/src/engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-transport/src/engine/stream_receiver_1
language: rust
---

# stream_receiver

Stream receiver loop

## Signature

```rust
fn stream_receiver(
        connections: Arc<RwLock<HashMap<NodeId, ConnectionState>>>,
        stats: Arc<RwLock<TransportStats>>,
        event_tx: broadcast::Sender<TransportEvent>,
        node_id: NodeId,
    )
```

## Docstring

Stream receiver loop

## Source
Lines 527–566 in `crates/oxide-transport/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-transport/src/engine.md) |
