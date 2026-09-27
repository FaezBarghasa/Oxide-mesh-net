---
okf_version: "0.2"
type: Function
title: intersects
description: "Bitwise AND test: returns true if any bit intersects"
resource: crates/oxide-acl/src/simd.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-acl"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-acl/src/simd/intersects_1
language: rust
---

# intersects

Bitwise AND test: returns true if any bit intersects

## Signature

```rust
pub fn intersects(&self, other: &Self) -> bool
```

## Decorators

- `inline(always)`

## Visibility

- `pub`

## Docstring

Bitwise AND test: returns true if any bit intersects
[inline(always)]

## Source
Lines 25–42 in `crates/oxide-acl/src/simd.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [simd](/crates/oxide-acl/src/simd.md) |
