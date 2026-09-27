---
okf_version: "0.2"
type: Class
title: CompiledRule
description: Compiled rule for fast SIMD evaluation
resource: crates/oxide-acl/src/rules.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-acl"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-acl/src/rules/CompiledRule
language: rust
---

# CompiledRule

Compiled rule for fast SIMD evaluation

## Signature

```rust
pub struct CompiledRule
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Compiled rule for fast SIMD evaluation
[derive(Debug, Clone)]

## Methods

- `identity_mask`
- `src_prefix_trie`
- `dst_prefix_trie`
- `protocol_bitmap`
- `src_port_bitmap`
- `dst_port_bitmap`

## Source
Lines 100–111 in `crates/oxide-acl/src/rules.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rules](/crates/oxide-acl/src/rules.md) |
