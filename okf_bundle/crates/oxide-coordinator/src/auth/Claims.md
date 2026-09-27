---
okf_version: "0.2"
type: Class
title: Claims
description: JWT claims for node authentication
resource: crates/oxide-coordinator/src/auth.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/auth/Claims
language: rust
---

# Claims

JWT claims for node authentication

## Signature

```rust
pub struct Claims
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

JWT claims for node authentication
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `sub`
- `mesh`
- `iat`
- `exp`
- `capabilities`
- `scopes`

## Source
Lines 14–21 in `crates/oxide-coordinator/src/auth.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [auth](/crates/oxide-coordinator/src/auth.md) |
