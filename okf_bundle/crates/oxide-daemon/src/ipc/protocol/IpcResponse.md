---
okf_version: "0.2"
type: Class
title: IpcResponse
description: Response messages returned from Daemon to CLI
resource: crates/oxide-daemon/src/ipc/protocol.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:33:11Z"
concept_id: crates/oxide-daemon/src/ipc/protocol/IpcResponse
language: rust
---

# IpcResponse

Response messages returned from Daemon to CLI

## Signature

```rust
pub enum IpcResponse
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Response messages returned from Daemon to CLI
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `message`
- `error`

## Source
Lines 24–31 in `crates/oxide-daemon/src/ipc/protocol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [protocol](/crates/oxide-daemon/src/ipc/protocol.md) |
