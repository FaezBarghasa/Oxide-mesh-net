---
okf_version: "0.2"
type: Class
title: L1DirectMappedCache
description: Thread-local L1 Direct-Mapped Routing Cache
resource: crates/oxide-transport/src/routing.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:27:12Z"
concept_id: crates/oxide-transport/src/routing/L1DirectMappedCache
language: rust
---

# L1DirectMappedCache

Thread-local L1 Direct-Mapped Routing Cache

## Signature

```rust
pub struct L1DirectMappedCache
```

## Visibility

- `pub`

## Docstring

Thread-local L1 Direct-Mapped Routing Cache

Designed for sub-2ns direct cache hits without heap allocation or atomics.

## Methods

- `entries`
- `valid`

## Source
Lines 127–130 in `crates/oxide-transport/src/routing.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [routing](/crates/oxide-transport/src/routing.md) |
