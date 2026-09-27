---
okf_version: "0.2"
type: Function
title: peers
description: Query connected peers
resource: crates/oxide-daemon/src/ipc/client.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:45:18Z"
concept_id: crates/oxide-daemon/src/ipc/client/peers_1
language: rust
---

# peers

Query connected peers

## Signature

```rust
pub fn peers(&mut self) -> Result<Vec<PeerStatusDto>, std::io::Error>
```

## Visibility

- `pub`

## Docstring

Query connected peers

## Source
Lines 76–82 in `crates/oxide-daemon/src/ipc/client.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [client](/crates/oxide-daemon/src/ipc/client.md) |
