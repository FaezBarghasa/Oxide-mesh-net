---
okf_version: "0.2"
type: Function
title: open_stream
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
concept_id: crates/oxide-transport/src/engine/open_stream
language: rust
---

# open_stream

Open a bidirectional stream

## Signature

```rust
impl TransportHandle { pub fn open_stream(&self, node_id: NodeId) -> Result<SendStream> }
```

## Visibility

- `pub`

## Docstring

Open a bidirectional stream

## Source
Lines 705–713 in `crates/oxide-transport/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-transport/src/engine.md) |
