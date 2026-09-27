---
okf_version: "0.2"
type: Class
title: MerkleAuditLogEntry
description: "[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]"
resource: crates/oxide-ui/src/models/audit.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-ui"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-26T16:53:23Z"
concept_id: crates/oxide-ui/src/models/audit/MerkleAuditLogEntry
language: rust
---

# MerkleAuditLogEntry

[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

## Signature

```rust
pub struct MerkleAuditLogEntry
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize, PartialEq)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

## Methods

- `sequence_num`
- `timestamp`
- `action_type`
- `initiator_node_id`
- `signature_hex`
- `merkle_leaf_hash`
- `root_hash_verified`

## Source
Lines 6–14 in `crates/oxide-ui/src/models/audit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [audit](/crates/oxide-ui/src/models/audit.md) |
