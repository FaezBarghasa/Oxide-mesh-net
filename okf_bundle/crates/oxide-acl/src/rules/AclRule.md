---
okf_version: "0.2"
type: Class
title: AclRule
description: ACL rule with compiled SIMD-friendly representation
resource: crates/oxide-acl/src/rules.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-acl"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-acl/src/rules/AclRule
language: rust
---

# AclRule

ACL rule with compiled SIMD-friendly representation

## Signature

```rust
pub struct AclRule
```

## Decorators

- `derive(Debug, Clone, serde::Serialize, serde::Deserialize)`

## Visibility

- `pub`

## Docstring

ACL rule with compiled SIMD-friendly representation
[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

## Methods

- `id`
- `action`
- `direction`
- `priority`
- `log`
- `src_identities`
- `src_prefixes`
- `dst_prefixes`
- `protocols`
- `src_ports`
- `dst_ports`
- `compiled`

## Source
Lines 74–96 in `crates/oxide-acl/src/rules.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rules](/crates/oxide-acl/src/rules.md) |
