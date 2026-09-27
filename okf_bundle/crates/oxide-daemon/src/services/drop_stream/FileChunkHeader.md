---
okf_version: "0.2"
type: Class
title: FileChunkHeader
description: File Chunk Header Metadata
resource: crates/oxide-daemon/src/services/drop_stream.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:45:36Z"
concept_id: crates/oxide-daemon/src/services/drop_stream/FileChunkHeader
language: rust
---

# FileChunkHeader

File Chunk Header Metadata

## Signature

```rust
pub struct FileChunkHeader
```

## Decorators

- `derive(Debug, Clone, serde::Serialize, serde::Deserialize)`

## Visibility

- `pub`

## Docstring

File Chunk Header Metadata
[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

## Methods

- `file_id`
- `chunk_index`
- `total_chunks`
- `chunk_size`
- `is_last`
- `chunk_blake3`

## Source
Lines 16–23 in `crates/oxide-daemon/src/services/drop_stream.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [drop_stream](/crates/oxide-daemon/src/services/drop_stream.md) |
