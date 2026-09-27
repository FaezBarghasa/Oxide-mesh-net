---
okf_version: "0.2"
type: Function
title: new
description: Create a new coordinator
resource: crates/oxide-coordinator/src/engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T12:04:02Z"
concept_id: crates/oxide-coordinator/src/engine/new
language: rust
---

# new

Create a new coordinator

## Signature

```rust
impl Coordinator { pub fn new(config: CoordinatorConfig) -> Result<Self> }
```

## Visibility

- `pub`

## Docstring

Create a new coordinator

## Source
Lines 30–41 in `crates/oxide-coordinator/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-coordinator/src/engine.md) |
