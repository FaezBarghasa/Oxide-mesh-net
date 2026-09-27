---
okf_version: "0.2"
type: Class
title: StaticRouteConfig
description: Static route configuration entry
resource: crates/oxide-daemon/src/config.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:27:46Z"
concept_id: crates/oxide-daemon/src/config/StaticRouteConfig
language: rust
---

# StaticRouteConfig

Static route configuration entry

## Signature

```rust
pub struct StaticRouteConfig
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Static route configuration entry
[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]

## Methods

- `prefix`
- `target_node`
- `pmtu`

## Source
Lines 9–13 in `crates/oxide-daemon/src/config.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [config](/crates/oxide-daemon/src/config.md) |
