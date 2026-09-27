---
okf_version: "0.2"
type: Function
title: bind
description: Bind and configure Unix domain socket with secure 0660 file permissions
resource: crates/oxide-daemon/src/ipc/socket.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:33:11Z"
concept_id: crates/oxide-daemon/src/ipc/socket/bind_1
language: rust
---

# bind

Bind and configure Unix domain socket with secure 0660 file permissions

## Signature

```rust
pub fn bind(&mut self) -> Result<(), std::io::Error>
```

## Visibility

- `pub`

## Docstring

Bind and configure Unix domain socket with secure 0660 file permissions

## Source
Lines 54–97 in `crates/oxide-daemon/src/ipc/socket.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [socket](/crates/oxide-daemon/src/ipc/socket.md) |
