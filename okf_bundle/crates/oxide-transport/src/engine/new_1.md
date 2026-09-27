---
okf_version: "0.2"
type: Function
title: new
description: Create a new transport engine
resource: crates/oxide-transport/src/engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-transport/src/engine/new_1
language: rust
---

# new

Create a new transport engine

## Signature

```rust
pub fn new(config: TransportConfig) -> Result<(Self, TransportHandle)>
```

## Visibility

- `pub`

## Docstring

Create a new transport engine

## Source
Lines 133–154 in `crates/oxide-transport/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-transport/src/engine.md) |
