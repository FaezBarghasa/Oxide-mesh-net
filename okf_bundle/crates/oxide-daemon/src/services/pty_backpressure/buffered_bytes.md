---
okf_version: "0.2"
type: Function
title: buffered_bytes
description: Current buffered byte count
resource: crates/oxide-daemon/src/services/pty_backpressure.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:33:11Z"
concept_id: crates/oxide-daemon/src/services/pty_backpressure/buffered_bytes
language: rust
---

# buffered_bytes

Current buffered byte count

## Signature

```rust
impl PtyStreamController { pub fn buffered_bytes(&self) -> usize }
```

## Visibility

- `pub`

## Docstring

Current buffered byte count

## Source
Lines 88–90 in `crates/oxide-daemon/src/services/pty_backpressure.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pty_backpressure](/crates/oxide-daemon/src/services/pty_backpressure.md) |
