---
okf_version: "0.2"
type: Function
title: authenticate_mqtt
description: Authenticate MQTT connection
resource: crates/oxide-coordinator/src/auth.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/auth/authenticate_mqtt
language: rust
---

# authenticate_mqtt

Authenticate MQTT connection

## Signature

```rust
impl AuthService { pub fn authenticate_mqtt(
        &self,
        connection: rumqttd::ConnectionId,
    ) -> Result<rumqttd::ConnectionId> }
```

## Visibility

- `pub`

## Docstring

Authenticate MQTT connection

## Source
Lines 109–117 in `crates/oxide-coordinator/src/auth.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [auth](/crates/oxide-coordinator/src/auth.md) |
