---
okf_version: "0.2"
type: Module
title: simd
description: SIMD-Vectorized Bitmask ACL Evaluation Engine
resource: crates/oxide-acl/src/simd.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-acl"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-acl/src/simd
language: rust
---

# simd

SIMD-Vectorized Bitmask ACL Evaluation Engine

## Docstring

SIMD-Vectorized Bitmask ACL Evaluation Engine

Compiles microsegmentation security rules into 256-bit SIMD-aligned bitmasks
evaluated in O(1) time (<5ns per packet) via AVX2 / ARM NEON / portable bitmasks.

## Relationships

| Type | Target |
|------|--------|
| related | [SimdBitmask256](/crates/oxide-acl/src/simd/SimdBitmask256.md) |
| related | [new](/crates/oxide-acl/src/simd/new.md) |
| related | [intersects](/crates/oxide-acl/src/simd/intersects.md) |
| related | [new](/crates/oxide-acl/src/simd/new.md) |
| related | [intersects](/crates/oxide-acl/src/simd/intersects.md) |
| related | [SimdCompiledMatrix](/crates/oxide-acl/src/simd/SimdCompiledMatrix.md) |
| related | [SimdEvaluator](/crates/oxide-acl/src/simd/SimdEvaluator.md) |
| related | [new](/crates/oxide-acl/src/simd/new.md) |
| related | [compile_rules](/crates/oxide-acl/src/simd/compile_rules.md) |
| related | [evaluate_batch](/crates/oxide-acl/src/simd/evaluate_batch.md) |
| related | [new](/crates/oxide-acl/src/simd/new.md) |
| related | [compile_rules](/crates/oxide-acl/src/simd/compile_rules.md) |
| related | [evaluate_batch](/crates/oxide-acl/src/simd/evaluate_batch.md) |
| related | [test_simd_bitmask_intersects](/crates/oxide-acl/src/simd/test_simd_bitmask_intersects.md) |
