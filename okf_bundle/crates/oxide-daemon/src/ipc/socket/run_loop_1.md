---
okf_version: "0.2"
type: Function
title: run_loop
description: "Run the server loop, accepting incoming connections and dispatching to handler"
resource: crates/oxide-daemon/src/ipc/socket.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:33:11Z"
concept_id: crates/oxide-daemon/src/ipc/socket/run_loop_1
language: rust
---

# run_loop

Run the server loop, accepting incoming connections and dispatching to handler

## Signature

```rust
pub fn run_loop(mut self, handler: Arc<H>) -> Result<(), std::io::Error>
```

## Type Parameters

- `H: IpcHandler`

## Visibility

- `pub`

## Docstring

Run the server loop, accepting incoming connections and dispatching to handler

## Source
Lines 100–120 in `crates/oxide-daemon/src/ipc/socket.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [socket](/crates/oxide-daemon/src/ipc/socket.md) |
