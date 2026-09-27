---
okf_version: "0.2"
type: Function
title: disconnect
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
concept_id: crates/oxide-transport/src/engine/disconnect
language: rust
---

# disconnect

Disconnect from a peer

## Signature

```rust
impl TransportHandle { pub fn disconnect(&self, node_id: NodeId) }
```

## Visibility

- `pub`

## Docstring

Disconnect from a peer

## Source
Lines 683–690 in `crates/oxide-transport/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-transport/src/engine.md) |
