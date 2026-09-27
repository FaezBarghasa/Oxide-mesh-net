---
okf_version: "0.2"
type: Function
title: verify_device_signature
description: Verify device identity signature
resource: crates/oxide-coordinator/src/auth.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/auth/verify_device_signature
language: rust
---

# verify_device_signature

Verify device identity signature

## Signature

```rust
impl AuthService { pub fn verify_device_signature(
        &self,
        public_key: &DeviceIdentityPublicKey,
        message: &[u8],
        signature: &oxide_crypto::keys::DeviceSignature,
    ) -> Result<()> }
```

## Visibility

- `pub`

## Docstring

Verify device identity signature

## Source
Lines 197–206 in `crates/oxide-coordinator/src/auth.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [auth](/crates/oxide-coordinator/src/auth.md) |
