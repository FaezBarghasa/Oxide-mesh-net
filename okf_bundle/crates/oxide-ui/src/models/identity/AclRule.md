---
okf_version: "0.2"
type: Class
title: AclRule
description: "[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]"
resource: crates/oxide-ui/src/models/identity.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-ui"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-26T16:52:29Z"
concept_id: crates/oxide-ui/src/models/identity/AclRule
language: rust
---

# AclRule

[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

## Signature

```rust
pub struct AclRule
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize, PartialEq)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

## Methods

- `id`
- `src_tag`
- `dst_tag`
- `proto_mask`
- `port_range_start`
- `port_range_end`
- `action_allow`

## Source
Lines 14–22 in `crates/oxide-ui/src/models/identity.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [identity](/crates/oxide-ui/src/models/identity.md) |
