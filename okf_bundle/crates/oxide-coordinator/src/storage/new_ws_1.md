---
okf_version: "0.2"
type: Function
title: new_ws
resource: crates/oxide-coordinator/src/storage.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/storage/new_ws_1
language: rust
---

# new_ws

## Signature

```rust
pub fn new_ws(
            url: &str,
            ns: &str,
            db_name: &str,
            user: Option<&str>,
            pass: Option<&str>,
        ) -> Result<Self>
```

## Visibility

- `pub`

## Source
Lines 323–355 in `crates/oxide-coordinator/src/storage.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [storage](/crates/oxide-coordinator/src/storage.md) |
| calls | [Storage](/crates/oxide-coordinator/src/storage/Storage.md) |
