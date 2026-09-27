---
okf_version: "0.2"
type: Class
title: DpiObfuscationConfig
description: "[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]"
resource: crates/oxide-ui/src/models/obfuscation.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-ui"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-26T16:51:53Z"
concept_id: crates/oxide-ui/src/models/obfuscation/DpiObfuscationConfig
language: rust
---

# DpiObfuscationConfig

[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

## Signature

```rust
pub struct DpiObfuscationConfig
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize, PartialEq)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

## Methods

- `magic_header_scramble_enabled`
- `custom_magic_hex`
- `padding_min_bytes`
- `padding_max_bytes`
- `pre_handshake_junk_enabled`
- `junk_burst_count`
- `junk_payload_max_bytes`

## Source
Lines 6–14 in `crates/oxide-ui/src/models/obfuscation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [obfuscation](/crates/oxide-ui/src/models/obfuscation.md) |
