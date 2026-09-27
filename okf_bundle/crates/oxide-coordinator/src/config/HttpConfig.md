---
okf_version: "0.2"
type: Class
title: HttpConfig
description: HTTP server configuration
resource: crates/oxide-coordinator/src/config.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/config/HttpConfig
language: rust
---

# HttpConfig

HTTP server configuration

## Signature

```rust
pub struct HttpConfig
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

HTTP server configuration
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `bind`
- `workers`
- `tls`
- `cert_path`
- `key_path`
- `request_timeout`
- `body_limit`
- `cors`
- `static_dir`

## Source
Lines 76–95 in `crates/oxide-coordinator/src/config.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [config](/crates/oxide-coordinator/src/config.md) |
