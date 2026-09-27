---
okf_version: "0.2"
type: Function
title: poll_probe
description: Check if a new probe frame should be dispatched
resource: crates/oxide-transport/src/dplpmtud.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:14:57Z"
concept_id: crates/oxide-transport/src/dplpmtud/poll_probe
language: rust
---

# poll_probe

Check if a new probe frame should be dispatched

## Signature

```rust
impl DplpmtudEngine { pub fn poll_probe(&mut self, now: Instant) -> Option<u16> }
```

## Visibility

- `pub`

## Docstring

Check if a new probe frame should be dispatched

## Source
Lines 87–137 in `crates/oxide-transport/src/dplpmtud.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dplpmtud](/crates/oxide-transport/src/dplpmtud.md) |
