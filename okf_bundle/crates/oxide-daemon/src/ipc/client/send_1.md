---
okf_version: "0.2"
type: Function
title: send
description: Send a request and wait for a response
resource: crates/oxide-daemon/src/ipc/client.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:45:18Z"
concept_id: crates/oxide-daemon/src/ipc/client/send_1
language: rust
---

# send

Send a request and wait for a response

## Signature

```rust
pub fn send(&mut self, req: &IpcRequest) -> Result<IpcResponse, std::io::Error>
```

## Visibility

- `pub`

## Docstring

Send a request and wait for a response

## Source
Lines 34–37 in `crates/oxide-daemon/src/ipc/client.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [client](/crates/oxide-daemon/src/ipc/client.md) |
| calls | [write_frame](/crates/oxide-daemon/src/ipc/protocol/write_frame.md) |
| calls | [read_frame](/crates/oxide-daemon/src/ipc/protocol/read_frame.md) |
