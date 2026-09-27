---
okf_version: "0.2"
type: Class
title: Protocol
description: Transport protocol
resource: crates/oxide-acl/src/rules.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-acl"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-acl/src/rules/Protocol
language: rust
---

# Protocol

Transport protocol

## Signature

```rust
pub enum Protocol
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)`

## Visibility

- `pub`

## Docstring

Transport protocol
[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]

## Source
Lines 28–34 in `crates/oxide-acl/src/rules.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rules](/crates/oxide-acl/src/rules.md) |
| called_by | [add_node_record](/crates/oxide-dns/src/engine/add_node_record.md) |
| called_by | [lookup](/crates/oxide-dns/src/engine/lookup.md) |
| called_by | [remove_node_record](/crates/oxide-dns/src/engine/remove_node_record.md) |
| called_by | [from_bytes](/crates/oxide-protocol/src/wire/from_bytes.md) |
| called_by | [new](/crates/oxide-protocol/src/wire/new.md) |
| called_by | [packet_type](/crates/oxide-protocol/src/wire/packet_type.md) |
| called_by | [validate](/crates/oxide-protocol/src/wire/validate.md) |
