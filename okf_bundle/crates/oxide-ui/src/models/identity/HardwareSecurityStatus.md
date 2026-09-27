---
okf_version: "0.2"
type: Class
title: HardwareSecurityStatus
description: "[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]"
resource: crates/oxide-ui/src/models/identity.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-ui"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-26T16:52:29Z"
concept_id: crates/oxide-ui/src/models/identity/HardwareSecurityStatus
language: rust
---

# HardwareSecurityStatus

[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

## Signature

```rust
pub struct HardwareSecurityStatus
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize, PartialEq)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

## Methods

- `hsm_type`
- `tpm2_bound`
- `cert_algo`
- `cert_serial`
- `expires_at`
- `coordinator_signed`

## Source
Lines 33–40 in `crates/oxide-ui/src/models/identity.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [identity](/crates/oxide-ui/src/models/identity.md) |
