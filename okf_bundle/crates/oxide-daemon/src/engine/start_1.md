---
okf_version: "0.2"
type: Function
title: start
description: Start the daemon and background watchdogs
resource: crates/oxide-daemon/src/engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:33:11Z"
concept_id: crates/oxide-daemon/src/engine/start_1
language: rust
---

# start

Start the daemon and background watchdogs

## Signature

```rust
pub fn start(engine: Arc<Self>) -> Result<(), Box<dyn std::error::Error>>
```

## Visibility

- `pub`

## Docstring

Start the daemon and background watchdogs

## Source
Lines 126–156 in `crates/oxide-daemon/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-daemon/src/engine.md) |
