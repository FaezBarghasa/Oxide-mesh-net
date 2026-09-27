---
okf_version: "0.2"
type: Function
title: remove
description: Remove a prefix route
resource: crates/oxide-transport/src/routing.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:27:12Z"
concept_id: crates/oxide-transport/src/routing/remove
language: rust
---

# remove

Remove a prefix route

## Signature

```rust
impl RadixRoutingTable { pub fn remove(&mut self, prefix: &OverlayPrefix) -> bool }
```

## Visibility

- `pub`

## Docstring

Remove a prefix route

## Source
Lines 75–88 in `crates/oxide-transport/src/routing.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [routing](/crates/oxide-transport/src/routing.md) |
