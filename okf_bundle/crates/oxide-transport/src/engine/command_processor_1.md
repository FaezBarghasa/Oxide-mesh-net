---
okf_version: "0.2"
type: Function
title: command_processor
description: Command processor loop
resource: crates/oxide-transport/src/engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-transport/src/engine/command_processor_1
language: rust
---

# command_processor

Command processor loop

## Signature

```rust
fn command_processor(
        connections: Arc<RwLock<HashMap<NodeId, ConnectionState>>>,
        endpoint_by_addr: Arc<RwLock<HashMap<SocketAddr, NodeId>>>,
        stats: Arc<RwLock<TransportStats>>,
        event_tx: broadcast::Sender<TransportEvent>,
        endpoint: Endpoint,
        _config: TransportConfig,
        mut command_rx: mpsc::UnboundedReceiver<TransportCommand>,
    )
```

## Docstring

Command processor loop

## Source
Lines 274–329 in `crates/oxide-transport/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-transport/src/engine.md) |
