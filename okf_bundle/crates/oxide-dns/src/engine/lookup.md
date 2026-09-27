---
okf_version: "0.2"
type: Function
title: lookup
resource: crates/oxide-dns/src/engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-dns"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-25T20:47:21Z"
concept_id: crates/oxide-dns/src/engine/lookup
language: rust
---

# lookup

## Signature

```rust
impl SplitHorizonResolver { pub fn lookup(&self, name: &str, record_type: RecordType) -> Result<Vec<Record>> }
```

## Visibility

- `pub`

## Source
Lines 351–365 in `crates/oxide-dns/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-dns/src/engine.md) |
| calls | [Protocol](/crates/oxide-acl/src/rules/Protocol.md) |
