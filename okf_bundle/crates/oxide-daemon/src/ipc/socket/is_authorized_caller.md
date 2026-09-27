---
okf_version: "0.2"
type: Function
title: is_authorized_caller
description: Validate caller credentials
resource: crates/oxide-daemon/src/ipc/socket.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:33:11Z"
concept_id: crates/oxide-daemon/src/ipc/socket/is_authorized_caller
language: rust
---

# is_authorized_caller

Validate caller credentials

## Signature

```rust
impl SecureIpcServer { pub fn is_authorized_caller(creds: &PeerCredentials) -> bool }
```

## Visibility

- `pub`

## Docstring

Validate caller credentials

## Source
Lines 165–179 in `crates/oxide-daemon/src/ipc/socket.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [socket](/crates/oxide-daemon/src/ipc/socket.md) |
