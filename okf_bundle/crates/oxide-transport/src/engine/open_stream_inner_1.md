---
okf_version: "0.2"
type: Function
title: open_stream_inner
description: Open a bidirectional stream
resource: crates/oxide-transport/src/engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-transport/src/engine/open_stream_inner_1
language: rust
---

# open_stream_inner

Open a bidirectional stream

## Signature

```rust
fn open_stream_inner(
        connections: &Arc<RwLock<HashMap<NodeId, ConnectionState>>>,
        node_id: NodeId,
    ) -> Result<SendStream>
```

## Docstring

Open a bidirectional stream

## Source
Lines 454–468 in `crates/oxide-transport/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-transport/src/engine.md) |
