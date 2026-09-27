---
okf_version: "0.2"
type: Function
title: validate_enrollment_token
description: Validate enrollment token
resource: crates/oxide-coordinator/src/auth.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/auth/validate_enrollment_token_1
language: rust
---

# validate_enrollment_token

Validate enrollment token

## Signature

```rust
pub fn validate_enrollment_token(&self, token: &str) -> Result<(String, Vec<String>)>
```

## Visibility

- `pub`

## Docstring

Validate enrollment token

## Source
Lines 120–152 in `crates/oxide-coordinator/src/auth.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [auth](/crates/oxide-coordinator/src/auth.md) |
