---
okf_version: "0.2"
type: Function
title: create_enrollment_token
description: Create enrollment token
resource: crates/oxide-coordinator/src/auth.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/auth/create_enrollment_token
language: rust
---

# create_enrollment_token

Create enrollment token

## Signature

```rust
impl AuthService { pub fn create_enrollment_token(
        &self,
        mesh_name: String,
        capabilities: Vec<String>,
        ttl: Option<Duration>,
    ) -> String }
```

## Visibility

- `pub`

## Docstring

Create enrollment token

## Source
Lines 164–194 in `crates/oxide-coordinator/src/auth.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [auth](/crates/oxide-coordinator/src/auth.md) |
