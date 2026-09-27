---
okf_version: "0.2"
type: Class
title: PacketMeta
description: Packet metadata for ACL evaluation
resource: crates/oxide-acl/src/rules.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-acl"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-acl/src/rules/PacketMeta
language: rust
---

# PacketMeta

Packet metadata for ACL evaluation

## Signature

```rust
pub struct PacketMeta
```

## Decorators

- `derive(Debug, Clone, Copy)`

## Visibility

- `pub`

## Docstring

Packet metadata for ACL evaluation
[derive(Debug, Clone, Copy)]

## Methods

- `src_ip`
- `dst_ip`
- `src_identity`
- `protocol`
- `src_port`
- `dst_port`
- `direction`

## Source
Lines 209–217 in `crates/oxide-acl/src/rules.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rules](/crates/oxide-acl/src/rules.md) |
