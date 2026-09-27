---
okf_version: "0.2"
type: Module
title: drop_stream
description: Oxide-Drop Zero-Copy File Chunk Streaming Engine
resource: crates/oxide-daemon/src/services/drop_stream.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:45:36Z"
concept_id: crates/oxide-daemon/src/services/drop_stream
language: rust
---

# drop_stream

Oxide-Drop Zero-Copy File Chunk Streaming Engine

## Docstring

Oxide-Drop Zero-Copy File Chunk Streaming Engine

Streams file payloads in bounded chunks governed by QUIC stream flow control windows
with BLAKE3 cryptographic verification.

## Relationships

| Type | Target |
|------|--------|
| related | [FileChunkHeader](/crates/oxide-daemon/src/services/drop_stream/FileChunkHeader.md) |
| related | [DropStreamer](/crates/oxide-daemon/src/services/drop_stream/DropStreamer.md) |
| related | [read_chunk](/crates/oxide-daemon/src/services/drop_stream/read_chunk.md) |
| related | [read_chunk](/crates/oxide-daemon/src/services/drop_stream/read_chunk.md) |
| related | [test_drop_chunk_streaming](/crates/oxide-daemon/src/services/drop_stream/test_drop_chunk_streaming.md) |
| related | [tracing](/_dependencies/cargo/tracing.md) |
