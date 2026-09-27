---
okf_version: "0.2"
type: Function
title: evaluate_scalar
description: Scalar evaluation fallback
resource: crates/oxide-acl/src/rules.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-acl"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-acl/src/rules/evaluate_scalar
language: rust
---

# evaluate_scalar

Scalar evaluation fallback

## Signature

```rust
impl AclEngine { fn evaluate_scalar(&self, meta: &PacketMeta) -> AclResult }
```

## Docstring

Scalar evaluation fallback

## Source
Lines 347–362 in `crates/oxide-acl/src/rules.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rules](/crates/oxide-acl/src/rules.md) |
