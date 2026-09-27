---
okf_version: "0.2"
type: Class
title: AclRuleRequest
description: "[derive(Deserialize, Serialize)]"
resource: crates/oxide-coordinator/src/handlers/acl.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/handlers/acl/AclRuleRequest
language: rust
---

# AclRuleRequest

[derive(Deserialize, Serialize)]

## Signature

```rust
pub struct AclRuleRequest
```

## Decorators

- `derive(Deserialize, Serialize)`

## Visibility

- `pub`

## Docstring

[derive(Deserialize, Serialize)]

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
Lines 20–30 in `crates/oxide-coordinator/src/handlers/acl.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [acl](/crates/oxide-coordinator/src/handlers/acl.md) |
