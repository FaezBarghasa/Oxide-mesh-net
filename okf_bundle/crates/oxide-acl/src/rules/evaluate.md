---
okf_version: "0.2"
type: Function
title: evaluate
description: Evaluate a packet against ACL rules
resource: crates/oxide-acl/src/rules.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-acl"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-acl/src/rules/evaluate
language: rust
---

# evaluate

Evaluate a packet against ACL rules

## Signature

```rust
impl AclEngine { pub fn evaluate(&self, meta: &PacketMeta) -> AclResult }
```

## Visibility

- `pub`

## Docstring

Evaluate a packet against ACL rules

## Source
Lines 336–344 in `crates/oxide-acl/src/rules.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rules](/crates/oxide-acl/src/rules.md) |
