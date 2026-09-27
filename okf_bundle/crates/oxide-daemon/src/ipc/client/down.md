---
okf_version: "0.2"
type: Function
title: down
description: Bring network down
resource: crates/oxide-daemon/src/ipc/client.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:45:18Z"
concept_id: crates/oxide-daemon/src/ipc/client/down
language: rust
---

# down

Bring network down

## Signature

```rust
impl IpcClient { pub fn down(&mut self) -> Result<String, std::io::Error> }
```

## Visibility

- `pub`

## Docstring

Bring network down

## Source
Lines 58–64 in `crates/oxide-daemon/src/ipc/client.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [client](/crates/oxide-daemon/src/ipc/client.md) |
