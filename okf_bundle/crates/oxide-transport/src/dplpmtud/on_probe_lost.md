---
okf_version: "0.2"
type: Function
title: on_probe_lost
description: Handle probe timeout / loss
resource: crates/oxide-transport/src/dplpmtud.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:14:57Z"
concept_id: crates/oxide-transport/src/dplpmtud/on_probe_lost
language: rust
---

# on_probe_lost

Handle probe timeout / loss

## Signature

```rust
impl DplpmtudEngine { pub fn on_probe_lost(&mut self, lost_size: u16) }
```

## Visibility

- `pub`

## Docstring

Handle probe timeout / loss

## Source
Lines 166–189 in `crates/oxide-transport/src/dplpmtud.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dplpmtud](/crates/oxide-transport/src/dplpmtud.md) |
