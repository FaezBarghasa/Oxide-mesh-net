---
okf_version: "0.2"
type: Function
title: write_frame
description: Write a length-prefixed JSON frame asynchronously
resource: crates/oxide-daemon/src/ipc/protocol.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:33:11Z"
concept_id: crates/oxide-daemon/src/ipc/protocol/write_frame
language: rust
---

# write_frame

Write a length-prefixed JSON frame asynchronously

## Signature

```rust
pub fn write_frame(
    writer: &mut W,
    msg: &T,
) -> std::io::Result<()>
```

## Type Parameters

- `W: AsyncWriteExt + Unpin`
- `T: Serialize`

## Visibility

- `pub`

## Docstring

Write a length-prefixed JSON frame asynchronously

## Source
Lines 72–82 in `crates/oxide-daemon/src/ipc/protocol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [protocol](/crates/oxide-daemon/src/ipc/protocol.md) |
| called_by | [send](/crates/oxide-daemon/src/ipc/client/send.md) |
| called_by | [test_ipc_framing_roundtrip](/crates/oxide-daemon/src/ipc/protocol/test_ipc_framing_roundtrip.md) |
| called_by | [handle_connection](/crates/oxide-daemon/src/ipc/socket/handle_connection.md) |
