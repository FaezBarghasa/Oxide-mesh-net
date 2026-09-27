---
okf_version: "0.2"
type: Function
title: validate_oidc_token
description: Validate OIDC token
resource: crates/oxide-coordinator/src/auth.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/auth/validate_oidc_token_1
language: rust
---

# validate_oidc_token

Validate OIDC token

## Signature

```rust
fn validate_oidc_token(&self, _token: &str) -> Result<(String, Vec<String>)>
```

## Docstring

Validate OIDC token

## Source
Lines 155–161 in `crates/oxide-coordinator/src/auth.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [auth](/crates/oxide-coordinator/src/auth.md) |
