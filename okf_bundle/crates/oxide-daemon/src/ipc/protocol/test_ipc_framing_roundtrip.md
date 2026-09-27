---
okf_version: "0.2"
type: Function
title: test_ipc_framing_roundtrip
description: "[tokio::test]"
resource: crates/oxide-daemon/src/ipc/protocol.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:33:11Z"
concept_id: crates/oxide-daemon/src/ipc/protocol/test_ipc_framing_roundtrip
language: rust
---

# test_ipc_framing_roundtrip

[tokio::test]

## Signature

```rust
fn test_ipc_framing_roundtrip()
```

## Decorators

- `tokio::test`

## Docstring

[tokio::test]

## Source
Lines 111–122 in `crates/oxide-daemon/src/ipc/protocol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [protocol](/crates/oxide-daemon/src/ipc/protocol.md) |
| calls | [write_frame](/crates/oxide-daemon/src/ipc/protocol/write_frame.md) |
| calls | [read_frame](/crates/oxide-daemon/src/ipc/protocol/read_frame.md) |
