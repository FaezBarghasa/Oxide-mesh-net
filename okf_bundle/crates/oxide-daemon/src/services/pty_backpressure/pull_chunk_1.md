---
okf_version: "0.2"
type: Function
title: pull_chunk
description: "Pull bytes for transmission over QUIC stream, releasing backpressure if below low watermark"
resource: crates/oxide-daemon/src/services/pty_backpressure.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:33:11Z"
concept_id: crates/oxide-daemon/src/services/pty_backpressure/pull_chunk_1
language: rust
---

# pull_chunk

Pull bytes for transmission over QUIC stream, releasing backpressure if below low watermark

## Signature

```rust
pub fn pull_chunk(&self, max_bytes: usize) -> Vec<u8>
```

## Visibility

- `pub`

## Docstring

Pull bytes for transmission over QUIC stream, releasing backpressure if below low watermark

## Source
Lines 63–80 in `crates/oxide-daemon/src/services/pty_backpressure.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pty_backpressure](/crates/oxide-daemon/src/services/pty_backpressure.md) |
