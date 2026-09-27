---
okf_version: "0.2"
type: Class
title: OidcConfig
description: OIDC configuration
resource: crates/oxide-coordinator/src/config.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/config/OidcConfig
language: rust
---

# OidcConfig

OIDC configuration

## Signature

```rust
pub struct OidcConfig
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

OIDC configuration
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `issuer_url`
- `client_id`
- `client_secret`
- `redirect_url`
- `scopes`
- `allowed_domains`

## Source
Lines 209–222 in `crates/oxide-coordinator/src/config.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [config](/crates/oxide-coordinator/src/config.md) |
