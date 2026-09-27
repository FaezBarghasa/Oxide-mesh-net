---
okf_version: "0.2"
type: Function
title: new_async
description: Asynchronously initialize storage according to configuration
resource: crates/oxide-coordinator/src/storage.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/storage/new_async_1
language: rust
---

# new_async

Asynchronously initialize storage according to configuration

## Signature

```rust
pub fn new_async(config: &StorageConfig) -> Result<Self>
```

## Visibility

- `pub`

## Docstring

Asynchronously initialize storage according to configuration

## Source
Lines 65–140 in `crates/oxide-coordinator/src/storage.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [storage](/crates/oxide-coordinator/src/storage.md) |
| calls | [Storage](/crates/oxide-coordinator/src/storage/Storage.md) |
