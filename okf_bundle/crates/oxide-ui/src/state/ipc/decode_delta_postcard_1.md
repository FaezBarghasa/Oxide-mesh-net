---
okf_version: "0.2"
type: Function
title: decode_delta_postcard
resource: crates/oxide-ui/src/state/ipc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-ui"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-26T16:55:06Z"
concept_id: crates/oxide-ui/src/state/ipc/decode_delta_postcard_1
language: rust
---

# decode_delta_postcard

## Signature

```rust
pub fn decode_delta_postcard(raw_bytes: &[u8]) -> Result<DaemonStateDelta, postcard::Error>
```

## Visibility

- `pub`

## Source
Lines 22–24 in `crates/oxide-ui/src/state/ipc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ipc](/crates/oxide-ui/src/state/ipc.md) |
