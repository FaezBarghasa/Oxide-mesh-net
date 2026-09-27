---
okf_version: "0.2"
type: Module
title: keys
description: Cryptographic key types and identity management
resource: crates/oxide-crypto/src/keys.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-crypto"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-crypto/src/keys
language: rust
---

# keys

Cryptographic key types and identity management

## Docstring

Cryptographic key types and identity management

## Relationships

| Type | Target |
|------|--------|
| related | [DeviceIdentityKey](/crates/oxide-crypto/src/keys/DeviceIdentityKey.md) |
| related | [zeroize](/crates/oxide-crypto/src/keys/zeroize.md) |
| related | [zeroize](/crates/oxide-crypto/src/keys/zeroize.md) |
| related | [drop](/crates/oxide-crypto/src/keys/drop.md) |
| related | [drop](/crates/oxide-crypto/src/keys/drop.md) |
| related | [generate](/crates/oxide-crypto/src/keys/generate.md) |
| related | [from_bytes](/crates/oxide-crypto/src/keys/from_bytes.md) |
| related | [public_key](/crates/oxide-crypto/src/keys/public_key.md) |
| related | [sign](/crates/oxide-crypto/src/keys/sign.md) |
| related | [to_bytes](/crates/oxide-crypto/src/keys/to_bytes.md) |
| related | [signing_key](/crates/oxide-crypto/src/keys/signing_key.md) |
| related | [generate](/crates/oxide-crypto/src/keys/generate.md) |
| related | [from_bytes](/crates/oxide-crypto/src/keys/from_bytes.md) |
| related | [public_key](/crates/oxide-crypto/src/keys/public_key.md) |
| related | [sign](/crates/oxide-crypto/src/keys/sign.md) |
| related | [to_bytes](/crates/oxide-crypto/src/keys/to_bytes.md) |
| related | [signing_key](/crates/oxide-crypto/src/keys/signing_key.md) |
| related | [fmt](/crates/oxide-crypto/src/keys/fmt.md) |
| related | [fmt](/crates/oxide-crypto/src/keys/fmt.md) |
| related | [eq](/crates/oxide-crypto/src/keys/eq.md) |
| related | [eq](/crates/oxide-crypto/src/keys/eq.md) |
| related | [DeviceIdentityPublicKey](/crates/oxide-crypto/src/keys/DeviceIdentityPublicKey.md) |
| related | [from_bytes](/crates/oxide-crypto/src/keys/from_bytes.md) |
| related | [verify](/crates/oxide-crypto/src/keys/verify.md) |
| related | [as_bytes](/crates/oxide-crypto/src/keys/as_bytes.md) |
| related | [to_bytes](/crates/oxide-crypto/src/keys/to_bytes.md) |
| related | [fingerprint](/crates/oxide-crypto/src/keys/fingerprint.md) |
| related | [from_bytes](/crates/oxide-crypto/src/keys/from_bytes.md) |
| related | [verify](/crates/oxide-crypto/src/keys/verify.md) |
| related | [as_bytes](/crates/oxide-crypto/src/keys/as_bytes.md) |
| related | [to_bytes](/crates/oxide-crypto/src/keys/to_bytes.md) |
| related | [fingerprint](/crates/oxide-crypto/src/keys/fingerprint.md) |
| related | [fmt](/crates/oxide-crypto/src/keys/fmt.md) |
| related | [fmt](/crates/oxide-crypto/src/keys/fmt.md) |
| related | [from_str](/crates/oxide-crypto/src/keys/from_str.md) |
| related | [from_str](/crates/oxide-crypto/src/keys/from_str.md) |
| related | [DeviceSignature](/crates/oxide-crypto/src/keys/DeviceSignature.md) |
| related | [to_bytes](/crates/oxide-crypto/src/keys/to_bytes.md) |
| related | [from_bytes](/crates/oxide-crypto/src/keys/from_bytes.md) |
| related | [to_bytes](/crates/oxide-crypto/src/keys/to_bytes.md) |
| related | [from_bytes](/crates/oxide-crypto/src/keys/from_bytes.md) |
| related | [KeyFingerprint](/crates/oxide-crypto/src/keys/KeyFingerprint.md) |
| related | [as_bytes](/crates/oxide-crypto/src/keys/as_bytes.md) |
| related | [to_hex](/crates/oxide-crypto/src/keys/to_hex.md) |
| related | [short](/crates/oxide-crypto/src/keys/short.md) |
| related | [as_bytes](/crates/oxide-crypto/src/keys/as_bytes.md) |
| related | [to_hex](/crates/oxide-crypto/src/keys/to_hex.md) |
| related | [short](/crates/oxide-crypto/src/keys/short.md) |
| related | [fmt](/crates/oxide-crypto/src/keys/fmt.md) |
| related | [fmt](/crates/oxide-crypto/src/keys/fmt.md) |
| related | [SessionKeyPair](/crates/oxide-crypto/src/keys/SessionKeyPair.md) |
| related | [zeroize](/crates/oxide-crypto/src/keys/zeroize.md) |
| related | [zeroize](/crates/oxide-crypto/src/keys/zeroize.md) |
| related | [generate](/crates/oxide-crypto/src/keys/generate.md) |
| related | [from_secret](/crates/oxide-crypto/src/keys/from_secret.md) |
| related | [public_key](/crates/oxide-crypto/src/keys/public_key.md) |
| related | [diffie_hellman](/crates/oxide-crypto/src/keys/diffie_hellman.md) |
| related | [to_bytes](/crates/oxide-crypto/src/keys/to_bytes.md) |
| related | [generate](/crates/oxide-crypto/src/keys/generate.md) |
| related | [from_secret](/crates/oxide-crypto/src/keys/from_secret.md) |
| related | [public_key](/crates/oxide-crypto/src/keys/public_key.md) |
| related | [diffie_hellman](/crates/oxide-crypto/src/keys/diffie_hellman.md) |
| related | [to_bytes](/crates/oxide-crypto/src/keys/to_bytes.md) |
| related | [fmt](/crates/oxide-crypto/src/keys/fmt.md) |
| related | [fmt](/crates/oxide-crypto/src/keys/fmt.md) |
| related | [SessionPublicKey](/crates/oxide-crypto/src/keys/SessionPublicKey.md) |
| related | [from_bytes](/crates/oxide-crypto/src/keys/from_bytes.md) |
| related | [as_bytes](/crates/oxide-crypto/src/keys/as_bytes.md) |
| related | [to_bytes](/crates/oxide-crypto/src/keys/to_bytes.md) |
| related | [from_bytes](/crates/oxide-crypto/src/keys/from_bytes.md) |
| related | [as_bytes](/crates/oxide-crypto/src/keys/as_bytes.md) |
| related | [to_bytes](/crates/oxide-crypto/src/keys/to_bytes.md) |
| related | [fmt](/crates/oxide-crypto/src/keys/fmt.md) |
| related | [fmt](/crates/oxide-crypto/src/keys/fmt.md) |
| related | [from_str](/crates/oxide-crypto/src/keys/from_str.md) |
| related | [from_str](/crates/oxide-crypto/src/keys/from_str.md) |
| related | [SharedSecret](/crates/oxide-crypto/src/keys/SharedSecret.md) |
| related | [from_bytes](/crates/oxide-crypto/src/keys/from_bytes.md) |
| related | [as_bytes](/crates/oxide-crypto/src/keys/as_bytes.md) |
| related | [derive_keys](/crates/oxide-crypto/src/keys/derive_keys.md) |
| related | [derive_aead_key](/crates/oxide-crypto/src/keys/derive_aead_key.md) |
| related | [from_bytes](/crates/oxide-crypto/src/keys/from_bytes.md) |
| related | [as_bytes](/crates/oxide-crypto/src/keys/as_bytes.md) |
| related | [derive_keys](/crates/oxide-crypto/src/keys/derive_keys.md) |
| related | [derive_aead_key](/crates/oxide-crypto/src/keys/derive_aead_key.md) |
| related | [fmt](/crates/oxide-crypto/src/keys/fmt.md) |
| related | [fmt](/crates/oxide-crypto/src/keys/fmt.md) |
| related | [AeadKey](/crates/oxide-crypto/src/keys/AeadKey.md) |
| related | [from_bytes](/crates/oxide-crypto/src/keys/from_bytes.md) |
| related | [as_bytes](/crates/oxide-crypto/src/keys/as_bytes.md) |
| related | [from_bytes](/crates/oxide-crypto/src/keys/from_bytes.md) |
| related | [as_bytes](/crates/oxide-crypto/src/keys/as_bytes.md) |
| related | [fmt](/crates/oxide-crypto/src/keys/fmt.md) |
| related | [fmt](/crates/oxide-crypto/src/keys/fmt.md) |
| related | [AeadNonce](/crates/oxide-crypto/src/keys/AeadNonce.md) |
| related | [from_bytes](/crates/oxide-crypto/src/keys/from_bytes.md) |
| related | [as_bytes](/crates/oxide-crypto/src/keys/as_bytes.md) |
| related | [increment](/crates/oxide-crypto/src/keys/increment.md) |
| related | [from_bytes](/crates/oxide-crypto/src/keys/from_bytes.md) |
| related | [as_bytes](/crates/oxide-crypto/src/keys/as_bytes.md) |
| related | [increment](/crates/oxide-crypto/src/keys/increment.md) |
| related | [blake3](/_dependencies/cargo/blake3.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
| related | [zeroize](/_dependencies/cargo/zeroize.md) |
