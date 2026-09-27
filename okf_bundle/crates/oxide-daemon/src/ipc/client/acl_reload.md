---
okf_version: "0.2"
type: Function
title: acl_reload
description: Reload ACL rules dynamically
resource: crates/oxide-daemon/src/ipc/client.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:45:18Z"
concept_id: crates/oxide-daemon/src/ipc/client/acl_reload
language: rust
---

# acl_reload

Reload ACL rules dynamically

## Signature

```rust
impl IpcClient { pub fn acl_reload(
        &mut self,
        rules_json: Option<String>,
    ) -> Result<String, std::io::Error> }
```

## Visibility

- `pub`

## Docstring

Reload ACL rules dynamically

## Source
Lines 85–94 in `crates/oxide-daemon/src/ipc/client.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [client](/crates/oxide-daemon/src/ipc/client.md) |
