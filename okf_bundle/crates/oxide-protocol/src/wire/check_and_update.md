---
okf_version: "0.2"
type: Function
title: check_and_update
description: Validate sequence number and update the sliding window.
resource: crates/oxide-protocol/src/wire.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/wire/check_and_update
language: rust
---

# check_and_update

Validate sequence number and update the sliding window.

## Signature

```rust
impl ReplayWindow128 { pub fn check_and_update(&mut self, seq: u64) -> bool }
```

## Visibility

- `pub`

## Docstring

Validate sequence number and update the sliding window.

Returns `true` if packet is valid (new or valid out-of-order), `false` if replayed or stale.
[inline]

## Source
Lines 331–359 in `crates/oxide-protocol/src/wire.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wire](/crates/oxide-protocol/src/wire.md) |
