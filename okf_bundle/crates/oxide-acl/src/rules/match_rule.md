---
okf_version: "0.2"
type: Function
title: match_rule
resource: crates/oxide-acl/src/rules.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-acl"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-acl/src/rules/match_rule
language: rust
---

# match_rule

## Signature

```rust
impl AclEngine { fn match_rule(&self, rule: &AclRule, meta: &PacketMeta) -> bool }
```

## Source
Lines 364–429 in `crates/oxide-acl/src/rules.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rules](/crates/oxide-acl/src/rules.md) |
