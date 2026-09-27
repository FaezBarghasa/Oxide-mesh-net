---
okf_version: "0.2"
type: Class
title: TlsConfig
description: TLS configuration
resource: crates/oxide-coordinator/src/config.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/config/TlsConfig
language: rust
---

# TlsConfig

TLS configuration

## Signature

```rust
pub struct TlsConfig
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

TLS configuration
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `cert_file`
- `key_file`
- `ca_file`
- `require_client_cert`

## Source
Lines 263–272 in `crates/oxide-coordinator/src/config.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [config](/crates/oxide-coordinator/src/config.md) |
