---
okf_version: "0.2"
type: Function
title: ws_telemetry_tunnel
description: WebSocket Fallback Tunnel endpoint disguised as telemetry stream
resource: crates/oxide-coordinator/src/handlers/tunnel_ws.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:33:11Z"
concept_id: crates/oxide-coordinator/src/handlers/tunnel_ws/ws_telemetry_tunnel
language: rust
---

# ws_telemetry_tunnel

WebSocket Fallback Tunnel endpoint disguised as telemetry stream

## Signature

```rust
pub fn ws_telemetry_tunnel(
    req: HttpRequest,
    body: web::Payload,
) -> Result<HttpResponse, actix_web::Error>
```

## Visibility

- `pub`

## Docstring

WebSocket Fallback Tunnel endpoint disguised as telemetry stream

## Source
Lines 13–81 in `crates/oxide-coordinator/src/handlers/tunnel_ws.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tunnel_ws](/crates/oxide-coordinator/src/handlers/tunnel_ws.md) |
| calls | [pre_parse_packet](/crates/oxide-protocol/src/wire/pre_parse_packet.md) |
