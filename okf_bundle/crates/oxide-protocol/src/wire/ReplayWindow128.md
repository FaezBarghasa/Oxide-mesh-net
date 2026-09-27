---
okf_version: "0.2"
type: Class
title: ReplayWindow128
description: 128-bit sliding bitmask replay window (RFC 6479 inspired)
resource: crates/oxide-protocol/src/wire.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/wire/ReplayWindow128
language: rust
---

# ReplayWindow128

128-bit sliding bitmask replay window (RFC 6479 inspired)

## Signature

```rust
pub struct ReplayWindow128
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

128-bit sliding bitmask replay window (RFC 6479 inspired)

Accepts legitimate out-of-order packets up to 128 packets behind the highest sequence,
while rejecting duplicate replays and packets older than the 128-packet window.
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Methods

- `last_seq`
- `window`

## Source
Lines 308–311 in `crates/oxide-protocol/src/wire.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wire](/crates/oxide-protocol/src/wire.md) |
