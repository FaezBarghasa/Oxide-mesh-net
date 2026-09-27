---
okf_version: "0.2"
type: Function
title: wait_for_shutdown
description: Wait for shutdown signal
resource: crates/oxide-coordinator/src/engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T12:04:02Z"
concept_id: crates/oxide-coordinator/src/engine/wait_for_shutdown
language: rust
---

# wait_for_shutdown

Wait for shutdown signal

## Signature

```rust
impl Coordinator { pub fn wait_for_shutdown(&mut self) }
```

## Visibility

- `pub`

## Docstring

Wait for shutdown signal

## Source
Lines 168–172 in `crates/oxide-coordinator/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-coordinator/src/engine.md) |
