---
okf_version: "0.2"
type: Function
title: add_rule
description: Add a rule dynamically
resource: crates/oxide-acl/src/rules.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-acl"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-acl/src/rules/add_rule
language: rust
---

# add_rule

Add a rule dynamically

## Signature

```rust
impl AclEngine { pub fn add_rule(&mut self, rule: AclRule) -> Result<()> }
```

## Visibility

- `pub`

## Docstring

Add a rule dynamically

## Source
Lines 432–446 in `crates/oxide-acl/src/rules.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rules](/crates/oxide-acl/src/rules.md) |
