---
okf_version: "0.2"
type: Function
title: update
description: Atomic RCU swap of a newly compiled routing table
resource: crates/oxide-transport/src/routing.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:27:12Z"
concept_id: crates/oxide-transport/src/routing/update_1
language: rust
---

# update

Atomic RCU swap of a newly compiled routing table

## Signature

```rust
pub fn update(&self, new_table: RadixRoutingTable)
```

## Visibility

- `pub`

## Docstring

Atomic RCU swap of a newly compiled routing table

## Source
Lines 233–235 in `crates/oxide-transport/src/routing.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [routing](/crates/oxide-transport/src/routing.md) |
