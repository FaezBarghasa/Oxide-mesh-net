---
okf_version: "0.2"
type: Class
title: PeerDeepDiagnostics
description: "[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]"
resource: crates/oxide-ui/src/models/peer.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-ui"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-26T16:50:46Z"
concept_id: crates/oxide-ui/src/models/peer/PeerDeepDiagnostics
language: rust
---

# PeerDeepDiagnostics

[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

## Signature

```rust
pub struct PeerDeepDiagnostics
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize, PartialEq)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

## Methods

- `peer_id`
- `quic_version`
- `congestion_algorithm`
- `bbr_pacing_rate_mbps`
- `bbr_inflight_bytes`
- `rss_socket_pool_index`
- `public_key_ed25519`
- `rekey_interval_secs`
- `tpm_endorsement_verified`
- `last_ping_rtt_us`

## Source
Lines 29–40 in `crates/oxide-ui/src/models/peer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [peer](/crates/oxide-ui/src/models/peer.md) |
