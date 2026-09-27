---
okf_version: "0.2"
type: Function
title: connect
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
concept_id: crates/oxide-transport/src/engine/connect
language: rust
---

# connect

Connect to a peer

## Signature

```rust
impl TransportHandle { pub fn connect(&self, node_id: NodeId, endpoint: SocketAddr) -> Result<()> }
```

## Visibility

- `pub`

## Docstring

Connect to a peer

## Source
Lines 671–680 in `crates/oxide-transport/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-transport/src/engine.md) |
