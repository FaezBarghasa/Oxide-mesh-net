---
okf_version: "0.2"
type: Function
title: status
description: Query daemon status
resource: crates/oxide-daemon/src/ipc/client.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:45:18Z"
concept_id: crates/oxide-daemon/src/ipc/client/status
language: rust
---

# status

Query daemon status

## Signature

```rust
impl IpcClient { pub fn status(&mut self) -> Result<DaemonStatusDto, std::io::Error> }
```

## Visibility

- `pub`

## Docstring

Query daemon status

## Source
Lines 40–46 in `crates/oxide-daemon/src/ipc/client.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [client](/crates/oxide-daemon/src/ipc/client.md) |
