---
okf_version: "0.2"
type: Function
title: on_probe_acked
description: Handle probe acknowledgment
resource: crates/oxide-transport/src/dplpmtud.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:14:57Z"
concept_id: crates/oxide-transport/src/dplpmtud/on_probe_acked_1
language: rust
---

# on_probe_acked

Handle probe acknowledgment

## Signature

```rust
pub fn on_probe_acked(&mut self, acked_size: u16, now: Instant)
```

## Visibility

- `pub`

## Docstring

Handle probe acknowledgment

## Source
Lines 140–163 in `crates/oxide-transport/src/dplpmtud.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dplpmtud](/crates/oxide-transport/src/dplpmtud.md) |
