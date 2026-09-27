---
okf_version: "0.2"
type: Function
title: connect
description: Connect to daemon Unix domain socket (with auto-fallback)
resource: crates/oxide-daemon/src/ipc/client.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:45:18Z"
concept_id: crates/oxide-daemon/src/ipc/client/connect_1
language: rust
---

# connect

Connect to daemon Unix domain socket (with auto-fallback)

## Signature

```rust
pub fn connect(custom_path: Option<&Path>) -> Result<Self, std::io::Error>
```

## Visibility

- `pub`

## Docstring

Connect to daemon Unix domain socket (with auto-fallback)

## Source
Lines 18–31 in `crates/oxide-daemon/src/ipc/client.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [client](/crates/oxide-daemon/src/ipc/client.md) |
