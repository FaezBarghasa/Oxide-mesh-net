---
okf_version: "0.2"
type: Function
title: connection_monitor
description: Monitor connection health and idle timeouts
resource: crates/oxide-transport/src/engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-transport/src/engine/connection_monitor
language: rust
---

# connection_monitor

Monitor connection health and idle timeouts

## Signature

```rust
impl TransportEngine { fn connection_monitor(
        connections: Arc<RwLock<HashMap<NodeId, ConnectionState>>>,
        endpoint_by_addr: Arc<RwLock<HashMap<SocketAddr, NodeId>>>,
        event_tx: broadcast::Sender<TransportEvent>,
        idle_timeout: Duration,
    ) }
```

## Docstring

Monitor connection health and idle timeouts

## Source
Lines 621–661 in `crates/oxide-transport/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-transport/src/engine.md) |
