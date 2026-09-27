---
okf_version: "0.2"
type: Class
title: RateLimitConfig
description: Rate limiting configuration
resource: crates/oxide-coordinator/src/config.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/config/RateLimitConfig
language: rust
---

# RateLimitConfig

Rate limiting configuration

## Signature

```rust
pub struct RateLimitConfig
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Rate limiting configuration
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `enabled`
- `requests_per_second`
- `burst`
- `mqtt_publish_per_second`

## Source
Lines 239–248 in `crates/oxide-coordinator/src/config.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [config](/crates/oxide-coordinator/src/config.md) |
