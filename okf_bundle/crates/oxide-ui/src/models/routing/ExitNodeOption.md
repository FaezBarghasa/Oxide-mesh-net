---
okf_version: "0.2"
type: Class
title: ExitNodeOption
description: "[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]"
resource: crates/oxide-ui/src/models/routing.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-ui"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-26T16:51:18Z"
concept_id: crates/oxide-ui/src/models/routing/ExitNodeOption
language: rust
---

# ExitNodeOption

[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

## Signature

```rust
pub struct ExitNodeOption
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize, PartialEq)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

## Methods

- `id`
- `hostname`
- `location_country`
- `location_city`
- `latency_ms`
- `is_active`
- `allows_lan_access`
- `verified_public_ip`
- `dns_leak_protected`

## Source
Lines 6–16 in `crates/oxide-ui/src/models/routing.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [routing](/crates/oxide-ui/src/models/routing.md) |
