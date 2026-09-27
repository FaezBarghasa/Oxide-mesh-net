---
okf_version: "0.2"
type: Function
title: compile_rules
description: Build SIMD evaluation matrices from compiled ACL rules
resource: crates/oxide-acl/src/simd.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-acl"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-acl/src/simd/compile_rules
language: rust
---

# compile_rules

Build SIMD evaluation matrices from compiled ACL rules

## Signature

```rust
impl SimdEvaluator { pub fn compile_rules(&mut self, rules: &[AclRule]) }
```

## Visibility

- `pub`

## Docstring

Build SIMD evaluation matrices from compiled ACL rules

## Source
Lines 69–121 in `crates/oxide-acl/src/simd.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [simd](/crates/oxide-acl/src/simd.md) |
