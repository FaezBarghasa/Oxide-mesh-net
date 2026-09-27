//! Oxide-Drop Zero-Copy File Chunk Streaming Engine
//!
//! Streams file payloads in bounded chunks governed by QUIC stream flow control windows
//! with BLAKE3 cryptographic verification.

use oxide_core::error::Result;
use std::path::Path;
use tokio::io::{AsyncReadExt, AsyncSeekExt, SeekFrom};
use tracing::debug;

/// Default transfer chunk size (64 KB for optimal QUIC throughput)
pub const DEFAULT_CHUNK_SIZE: usize = 64 * 1024;

/// File Chunk Header Metadata
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileChunkHeader {
    pub file_id: String,
    pub chunk_index: u64,
    pub total_chunks: u64,
    pub chunk_size: u32,
    pub is_last: bool,
    pub chunk_blake3: [u8; 32],
}

/// Zero-Copy File Streamer
pub struct DropStreamer;

impl DropStreamer {
    /// Read a specific chunk from disk with BLAKE3 checksum calculation
    pub async fn read_chunk(
        file_path: &Path,
        file_id: String,
        chunk_index: u64,
        chunk_size: usize,
    ) -> Result<(FileChunkHeader, Vec<u8>)> {
        let mut file = tokio::fs::File::open(file_path)
            .await
            .map_err(oxide_core::error::OxideError::Io)?;

        let file_len = file
            .metadata()
            .await
            .map_err(oxide_core::error::OxideError::Io)?
            .len();
        let total_chunks = file_len.div_ceil(chunk_size as u64);

        let offset = chunk_index * chunk_size as u64;
        file.seek(SeekFrom::Start(offset))
            .await
            .map_err(oxide_core::error::OxideError::Io)?;

        let mut buffer = vec![0u8; chunk_size];
        let bytes_read = file
            .read(&mut buffer)
            .await
            .map_err(oxide_core::error::OxideError::Io)?;
        buffer.truncate(bytes_read);

        let hash = blake3::hash(&buffer);
        let is_last = chunk_index + 1 >= total_chunks;

        let header = FileChunkHeader {
            file_id,
            chunk_index,
            total_chunks,
            chunk_size: bytes_read as u32,
            is_last,
            chunk_blake3: *hash.as_bytes(),
        };

        debug!(
            "Read Oxide-Drop chunk {}/{} ({} bytes)",
            chunk_index + 1,
            total_chunks,
            bytes_read
        );
        Ok((header, buffer))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn test_drop_chunk_streaming() {
        let temp_path = std::env::temp_dir().join("oxide_drop_test.dat");
        {
            let mut file = tokio::fs::File::create(&temp_path).await.unwrap();
            file.write_all(&vec![0xAA; 100 * 1024]).await.unwrap(); // 100 KB
        }

        let (header, data) =
            DropStreamer::read_chunk(&temp_path, "file-42".to_string(), 0, 64 * 1024)
                .await
                .unwrap();

        assert_eq!(header.chunk_index, 0);
        assert_eq!(header.total_chunks, 2);
        assert_eq!(data.len(), 64 * 1024);
        assert!(!header.is_last);

        let _ = tokio::fs::remove_file(temp_path).await;
    }
}
