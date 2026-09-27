---
okf_version: "0.2"
type: Function
title: disconnect_inner
description: Disconnect from a peer
resource: crates/oxide-transport/src/engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-transport/src/engine/disconnect_inner_1
language: rust
---

# disconnect_inner

Disconnect from a peer

## Signature

```rust
fn disconnect_inner(
        connections: &Arc<RwLock<HashMap<NodeId, ConnectionState>>>,
        endpoint_by_addr: &Arc<RwLock<HashMap<SocketAddr, NodeId>>>,
        node_id: NodeId,
    )
```

## Docstring

Disconnect from a peer

## Source
Lines 409–420 in `crates/oxide-transport/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-transport/src/engine.md) |
