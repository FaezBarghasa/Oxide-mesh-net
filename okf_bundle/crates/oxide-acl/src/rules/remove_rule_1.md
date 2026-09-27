---
okf_version: "0.2"
type: Function
title: remove_rule
description: Remove a rule by ID
resource: crates/oxide-acl/src/rules.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-acl"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-acl/src/rules/remove_rule_1
language: rust
---

# remove_rule

Remove a rule by ID

## Signature

```rust
pub fn remove_rule(&mut self, rule_id: &str) -> Result<()>
```

## Visibility

- `pub`

## Docstring

Remove a rule by ID

## Source
Lines 449–457 in `crates/oxide-acl/src/rules.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rules](/crates/oxide-acl/src/rules.md) |
