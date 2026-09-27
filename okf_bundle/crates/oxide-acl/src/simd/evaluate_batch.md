---
okf_version: "0.2"
type: Function
title: evaluate_batch
description: Constant-time SIMD batch evaluation
resource: crates/oxide-acl/src/simd.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-acl"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-acl/src/simd/evaluate_batch
language: rust
---

# evaluate_batch

Constant-time SIMD batch evaluation

## Signature

```rust
impl SimdEvaluator { pub fn evaluate_batch(&self, _rules: &[AclRule], meta: &PacketMeta) -> Option<AclResult> }
```

## Visibility

- `pub`

## Docstring

Constant-time SIMD batch evaluation
[inline(always)]

## Source
Lines 125–166 in `crates/oxide-acl/src/simd.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [simd](/crates/oxide-acl/src/simd.md) |
