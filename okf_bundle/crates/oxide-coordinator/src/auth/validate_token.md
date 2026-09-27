---
okf_version: "0.2"
type: Function
title: validate_token
description: Validate a JWT token
resource: crates/oxide-coordinator/src/auth.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/auth/validate_token
language: rust
---

# validate_token

Validate a JWT token

## Signature

```rust
impl AuthService { pub fn validate_token(&self, token: &str) -> Result<Claims> }
```

## Visibility

- `pub`

## Docstring

Validate a JWT token

## Source
Lines 102–106 in `crates/oxide-coordinator/src/auth.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [auth](/crates/oxide-coordinator/src/auth.md) |
