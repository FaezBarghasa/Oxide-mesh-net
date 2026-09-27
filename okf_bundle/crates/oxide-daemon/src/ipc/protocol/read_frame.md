---
okf_version: "0.2"
type: Function
title: read_frame
description: Read a length-prefixed JSON frame asynchronously
resource: crates/oxide-daemon/src/ipc/protocol.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:33:11Z"
concept_id: crates/oxide-daemon/src/ipc/protocol/read_frame
language: rust
---

# read_frame

Read a length-prefixed JSON frame asynchronously

## Signature

```rust
pub fn read_frame(
    reader: &mut R,
) -> std::io::Result<T>
```

## Type Parameters

- `R: AsyncReadExt + Unpin`
- `T: DeserializeOwned`

## Visibility

- `pub`

## Docstring

Read a length-prefixed JSON frame asynchronously

## Source
Lines 85–103 in `crates/oxide-daemon/src/ipc/protocol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [protocol](/crates/oxide-daemon/src/ipc/protocol.md) |
| called_by | [send](/crates/oxide-daemon/src/ipc/client/send.md) |
| called_by | [test_ipc_framing_roundtrip](/crates/oxide-daemon/src/ipc/protocol/test_ipc_framing_roundtrip.md) |
| called_by | [handle_connection](/crates/oxide-daemon/src/ipc/socket/handle_connection.md) |
