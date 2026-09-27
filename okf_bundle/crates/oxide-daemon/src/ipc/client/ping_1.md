---
okf_version: "0.2"
type: Function
title: ping
description: Ping daemon
resource: crates/oxide-daemon/src/ipc/client.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:45:18Z"
concept_id: crates/oxide-daemon/src/ipc/client/ping_1
language: rust
---

# ping

Ping daemon

## Signature

```rust
pub fn ping(&mut self) -> Result<bool, std::io::Error>
```

## Visibility

- `pub`

## Docstring

Ping daemon

## Source
Lines 97–102 in `crates/oxide-daemon/src/ipc/client.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [client](/crates/oxide-daemon/src/ipc/client.md) |
