---
okf_version: "0.2"
type: Class
title: AclRuleWire
description: Wire format for ACL rule
resource: crates/oxide-protocol/src/wire.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/wire/AclRuleWire
language: rust
---

# AclRuleWire

Wire format for ACL rule

## Signature

```rust
pub struct AclRuleWire
```

## Decorators

- `derive(Debug, Clone, serde::Serialize, serde::Deserialize)`

## Visibility

- `pub`

## Docstring

Wire format for ACL rule
[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

## Methods

- `action`
- `src_identity`
- `dst_prefix`
- `protocol`
- `port_range`
- `direction`

## Source
Lines 216–223 in `crates/oxide-protocol/src/wire.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wire](/crates/oxide-protocol/src/wire.md) |
