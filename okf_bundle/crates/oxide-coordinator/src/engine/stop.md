---
okf_version: "0.2"
type: Function
title: stop
description: Stop the coordinator
resource: crates/oxide-coordinator/src/engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T12:04:02Z"
concept_id: crates/oxide-coordinator/src/engine/stop
language: rust
---

# stop

Stop the coordinator

## Signature

```rust
impl Coordinator { pub fn stop(&mut self) -> Result<()> }
```

## Visibility

- `pub`

## Docstring

Stop the coordinator

## Source
Lines 156–165 in `crates/oxide-coordinator/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-coordinator/src/engine.md) |
