---
okf_version: "0.2"
type: Function
title: match_ip
resource: crates/oxide-acl/src/rules.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-acl"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-acl/src/rules/match_ip
language: rust
---

# match_ip

## Signature

```rust
impl PrefixTrie { fn match_ip(&self, ip: OverlayIp) -> SmallVec<[OverlayPrefix; 4]> }
```

## Source
Lines 161–195 in `crates/oxide-acl/src/rules.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rules](/crates/oxide-acl/src/rules.md) |
