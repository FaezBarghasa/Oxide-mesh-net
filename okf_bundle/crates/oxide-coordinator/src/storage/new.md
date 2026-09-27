---
okf_version: "0.2"
type: Function
title: new
description: Synchronous initialization wrapper (defaults to in-memory or blocks on async runtime)
resource: crates/oxide-coordinator/src/storage.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/storage/new
language: rust
---

# new

Synchronous initialization wrapper (defaults to in-memory or blocks on async runtime)

## Signature

```rust
impl Storage { pub fn new(config: &StorageConfig) -> Result<Self> }
```

## Visibility

- `pub`

## Docstring

Synchronous initialization wrapper (defaults to in-memory or blocks on async runtime)

## Source
Lines 143–147 in `crates/oxide-coordinator/src/storage.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [storage](/crates/oxide-coordinator/src/storage.md) |
