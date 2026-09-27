---
okf_version: "0.2"
type: Class
title: DnsRecord
description: DNS record payload
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/DnsRecord
language: rust
---

# DnsRecord

DNS record payload

## Signature

```rust
pub struct DnsRecord
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

DNS record payload
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `name`
- `record_type`
- `data`
- `ttl`
- `source`

## Source
Lines 451–457 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
