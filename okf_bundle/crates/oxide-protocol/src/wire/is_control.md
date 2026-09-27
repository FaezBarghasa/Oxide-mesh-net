---
okf_version: "0.2"
type: Function
title: is_control
description: Check if packet is control plane
resource: crates/oxide-protocol/src/wire.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/wire/is_control
language: rust
---

# is_control

Check if packet is control plane

## Signature

```rust
impl WirePacket { pub fn is_control(&self) -> bool }
```

## Visibility

- `pub`

## Docstring

Check if packet is control plane

## Source
Lines 124–129 in `crates/oxide-protocol/src/wire.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wire](/crates/oxide-protocol/src/wire.md) |
