---
okf_version: "0.2"
type: Function
title: connect_inner
description: Connect to a peer
resource: crates/oxide-transport/src/engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-transport/src/engine/connect_inner_1
language: rust
---

# connect_inner

Connect to a peer

## Signature

```rust
fn connect_inner(
        connections: &Arc<RwLock<HashMap<NodeId, ConnectionState>>>,
        endpoint_by_addr: &Arc<RwLock<HashMap<SocketAddr, NodeId>>>,
        stats: &Arc<RwLock<TransportStats>>,
        event_tx: &broadcast::Sender<TransportEvent>,
        endpoint: &Endpoint,
        node_id: NodeId,
        addr: SocketAddr,
    ) -> Result<()>
```

## Docstring

Connect to a peer

## Source
Lines 332–406 in `crates/oxide-transport/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-transport/src/engine.md) |
