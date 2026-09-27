---
okf_version: "0.2"
type: Class
title: PeerCardData
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
concept_id: crates/oxide-ui/src/models/peer/PeerCardData
language: rust
---

# PeerCardData

[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

## Signature

```rust
pub struct PeerCardData
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize, PartialEq)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

## Methods

- `id`
- `hostname`
- `overlay_ip`
- `os`
- `connection_vector`
- `remote_endpoint`
- `rtt_ms`
- `last_handshake_secs`
- `bytes_rx`
- `bytes_tx`
- `is_online`
- `tags`

## Source
Lines 13–26 in `crates/oxide-ui/src/models/peer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [peer](/crates/oxide-ui/src/models/peer.md) |
