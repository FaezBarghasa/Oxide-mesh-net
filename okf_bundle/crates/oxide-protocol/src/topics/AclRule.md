---
okf_version: "0.2"
type: Class
title: AclRule
description: ACL rule
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/AclRule
language: rust
---

# AclRule

ACL rule

## Signature

```rust
pub struct AclRule
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

ACL rule
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `id`
- `action`
- `src_identities`
- `dst_prefixes`
- `protocols`
- `port_ranges`
- `direction`
- `log`
- `priority`

## Source
Lines 437–447 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
