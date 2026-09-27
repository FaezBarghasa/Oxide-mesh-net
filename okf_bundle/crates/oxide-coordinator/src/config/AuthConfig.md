---
okf_version: "0.2"
type: Class
title: AuthConfig
description: Authentication configuration
resource: crates/oxide-coordinator/src/config.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/config/AuthConfig
language: rust
---

# AuthConfig

Authentication configuration

## Signature

```rust
pub struct AuthConfig
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Authentication configuration
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `enabled`
- `jwt_secret`
- `jwt_expiration`
- `oidc`
- `enrollment_tokens`

## Source
Lines 182–193 in `crates/oxide-coordinator/src/config.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [config](/crates/oxide-coordinator/src/config.md) |
