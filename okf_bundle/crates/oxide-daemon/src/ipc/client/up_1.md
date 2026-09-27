---
okf_version: "0.2"
type: Function
title: up
description: Bring network up
resource: crates/oxide-daemon/src/ipc/client.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:45:18Z"
concept_id: crates/oxide-daemon/src/ipc/client/up_1
language: rust
---

# up

Bring network up

## Signature

```rust
pub fn up(&mut self, config_path: Option<String>) -> Result<String, std::io::Error>
```

## Visibility

- `pub`

## Docstring

Bring network up

## Source
Lines 49–55 in `crates/oxide-daemon/src/ipc/client.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [client](/crates/oxide-daemon/src/ipc/client.md) |
