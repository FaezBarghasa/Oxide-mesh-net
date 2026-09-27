---
okf_version: "0.2"
type: Function
title: decode_delta_json
resource: crates/oxide-ui/src/state/ipc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-ui"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-26T16:55:06Z"
concept_id: crates/oxide-ui/src/state/ipc/decode_delta_json
language: rust
---

# decode_delta_json

## Signature

```rust
impl StateReconciler { pub fn decode_delta_json(json_str: &str) -> Result<DaemonStateDelta, serde_json::Error> }
```

## Visibility

- `pub`

## Source
Lines 26–28 in `crates/oxide-ui/src/state/ipc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ipc](/crates/oxide-ui/src/state/ipc.md) |
