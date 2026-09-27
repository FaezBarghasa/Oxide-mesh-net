---
okf_version: "0.2"
type: Function
title: routes
description: Query active routing table
resource: crates/oxide-daemon/src/ipc/client.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:45:18Z"
concept_id: crates/oxide-daemon/src/ipc/client/routes
language: rust
---

# routes

Query active routing table

## Signature

```rust
impl IpcClient { pub fn routes(&mut self) -> Result<Vec<RouteEntryDto>, std::io::Error> }
```

## Visibility

- `pub`

## Docstring

Query active routing table

## Source
Lines 67–73 in `crates/oxide-daemon/src/ipc/client.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [client](/crates/oxide-daemon/src/ipc/client.md) |
