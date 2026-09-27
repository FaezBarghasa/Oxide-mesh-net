---
okf_version: "0.2"
type: Function
title: push_chunk
description: "Push PTY chunk into buffer, evaluating backpressure"
resource: crates/oxide-daemon/src/services/pty_backpressure.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:33:11Z"
concept_id: crates/oxide-daemon/src/services/pty_backpressure/push_chunk_1
language: rust
---

# push_chunk

Push PTY chunk into buffer, evaluating backpressure

## Signature

```rust
pub fn push_chunk(&self, chunk: &[u8]) -> bool
```

## Visibility

- `pub`

## Docstring

Push PTY chunk into buffer, evaluating backpressure

## Source
Lines 41–60 in `crates/oxide-daemon/src/services/pty_backpressure.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pty_backpressure](/crates/oxide-daemon/src/services/pty_backpressure.md) |
