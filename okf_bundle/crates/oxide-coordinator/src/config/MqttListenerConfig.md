---
okf_version: "0.2"
type: Class
title: MqttListenerConfig
description: MQTT listener configuration
resource: crates/oxide-coordinator/src/config.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/config/MqttListenerConfig
language: rust
---

# MqttListenerConfig

MQTT listener configuration

## Signature

```rust
pub struct MqttListenerConfig
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

MQTT listener configuration
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `bind`
- `tls`
- `cert_path`
- `key_path`
- `max_connections`
- `max_packet_size`

## Source
Lines 46–59 in `crates/oxide-coordinator/src/config.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [config](/crates/oxide-coordinator/src/config.md) |
