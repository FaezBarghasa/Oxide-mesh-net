---
okf_version: "0.2"
type: Class
title: PeerCredentials
description: Caller credential metadata
resource: crates/oxide-daemon/src/ipc/socket.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:33:11Z"
concept_id: crates/oxide-daemon/src/ipc/socket/PeerCredentials
language: rust
---

# PeerCredentials

Caller credential metadata

## Signature

```rust
pub struct PeerCredentials
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Caller credential metadata
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Methods

- `uid`
- `gid`
- `pid`

## Source
Lines 19–23 in `crates/oxide-daemon/src/ipc/socket.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [socket](/crates/oxide-daemon/src/ipc/socket.md) |
