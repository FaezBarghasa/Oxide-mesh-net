---
okf_version: "0.2"
type: Class
title: IngressService
description: "[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]"
resource: crates/oxide-ui/src/models/services.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-ui"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-26T16:52:58Z"
concept_id: crates/oxide-ui/src/models/services/IngressService
language: rust
---

# IngressService

[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

## Signature

```rust
pub struct IngressService
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize, PartialEq)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

## Methods

- `service_id`
- `local_bind_addr`
- `magic_dns_alias`
- `is_public_funnel`
- `public_url`
- `acme_tls_provisioned`
- `requests_handled`

## Source
Lines 38–46 in `crates/oxide-ui/src/models/services.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [services](/crates/oxide-ui/src/models/services.md) |
