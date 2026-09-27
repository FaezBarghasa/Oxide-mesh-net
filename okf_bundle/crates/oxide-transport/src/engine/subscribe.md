---
okf_version: "0.2"
type: Function
title: subscribe
description: Subscribe to transport events
resource: crates/oxide-transport/src/engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-transport/src/engine/subscribe
language: rust
---

# subscribe

Subscribe to transport events

## Signature

```rust
impl TransportEngine { pub fn subscribe(&self) -> broadcast::Receiver<TransportEvent> }
```

## Visibility

- `pub`

## Docstring

Subscribe to transport events

## Source
Lines 269–271 in `crates/oxide-transport/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-transport/src/engine.md) |
