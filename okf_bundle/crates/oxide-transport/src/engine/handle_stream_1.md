---
okf_version: "0.2"
type: Function
title: handle_stream
description: Handle a single stream
resource: crates/oxide-transport/src/engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-transport/src/engine/handle_stream_1
language: rust
---

# handle_stream

Handle a single stream

## Signature

```rust
fn handle_stream(
        _send: SendStream,
        mut recv: RecvStream,
        stats: Arc<RwLock<TransportStats>>,
        event_tx: broadcast::Sender<TransportEvent>,
        node_id: NodeId,
        stream_id: u64,
    )
```

## Docstring

Handle a single stream

## Source
Lines 569–618 in `crates/oxide-transport/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-transport/src/engine.md) |
