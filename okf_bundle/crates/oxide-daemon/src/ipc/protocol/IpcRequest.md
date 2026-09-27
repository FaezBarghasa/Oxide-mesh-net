---
okf_version: "0.2"
type: Class
title: IpcRequest
description: Request messages sent from CLI to Daemon
resource: crates/oxide-daemon/src/ipc/protocol.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:33:11Z"
concept_id: crates/oxide-daemon/src/ipc/protocol/IpcRequest
language: rust
---

# IpcRequest

Request messages sent from CLI to Daemon

## Signature

```rust
pub enum IpcRequest
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Request messages sent from CLI to Daemon
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `config_path`
- `rules_json`

## Source
Lines 12–20 in `crates/oxide-daemon/src/ipc/protocol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [protocol](/crates/oxide-daemon/src/ipc/protocol.md) |
