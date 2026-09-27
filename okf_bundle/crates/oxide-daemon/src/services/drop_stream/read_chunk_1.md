---
okf_version: "0.2"
type: Function
title: read_chunk
description: Read a specific chunk from disk with BLAKE3 checksum calculation
resource: crates/oxide-daemon/src/services/drop_stream.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:45:36Z"
concept_id: crates/oxide-daemon/src/services/drop_stream/read_chunk_1
language: rust
---

# read_chunk

Read a specific chunk from disk with BLAKE3 checksum calculation

## Signature

```rust
pub fn read_chunk(
        file_path: &Path,
        file_id: String,
        chunk_index: u64,
        chunk_size: usize,
    ) -> Result<(FileChunkHeader, Vec<u8>)>
```

## Visibility

- `pub`

## Docstring

Read a specific chunk from disk with BLAKE3 checksum calculation

## Source
Lines 30–78 in `crates/oxide-daemon/src/services/drop_stream.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [drop_stream](/crates/oxide-daemon/src/services/drop_stream.md) |
