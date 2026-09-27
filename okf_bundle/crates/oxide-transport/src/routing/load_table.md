---
okf_version: "0.2"
type: Function
title: load_table
description: Lock-free load of the entire routing table Arc
resource: crates/oxide-transport/src/routing.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:27:12Z"
concept_id: crates/oxide-transport/src/routing/load_table
language: rust
---

# load_table

Lock-free load of the entire routing table Arc

## Signature

```rust
impl RcuRouter { pub fn load_table(&self) -> arc_swap::Guard<Arc<RadixRoutingTable>> }
```

## Visibility

- `pub`

## Docstring

Lock-free load of the entire routing table Arc
[inline]

## Source
Lines 228–230 in `crates/oxide-transport/src/routing.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [routing](/crates/oxide-transport/src/routing.md) |
