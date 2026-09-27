---
okf_version: "0.2"
type: Function
title: start
description: Start the transport engine
resource: crates/oxide-transport/src/engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-transport/src/engine/start
language: rust
---

# start

Start the transport engine

## Signature

```rust
impl TransportEngine { pub fn start(&mut self) -> Result<()> }
```

## Visibility

- `pub`

## Docstring

Start the transport engine

## Source
Lines 157–266 in `crates/oxide-transport/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-transport/src/engine.md) |
| calls | [make_client_config](/crates/oxide-transport/src/config/make_client_config.md) |
| calls | [make_server_config](/crates/oxide-transport/src/config/make_server_config.md) |
