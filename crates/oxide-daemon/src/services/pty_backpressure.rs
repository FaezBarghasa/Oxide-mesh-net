//! Oxide-SSH Pseudo-Terminal (PTY) Backpressure & Bounded Ring Buffer
//!
//! Enforces a strict 256 KB memory bound on PTY terminal buffers, suspending reads
//! when the remote stream buffer is saturated to prevent runaway memory exhaustion.

use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, warn};

/// Maximum PTY ring buffer capacity (256 KB)
pub const MAX_PTY_BUFFER_BYTES: usize = 256 * 1024;

/// High watermark (90%) for triggering read suspension
pub const HIGH_WATERMARK_BYTES: usize = (MAX_PTY_BUFFER_BYTES * 9) / 10;

/// Low watermark (50%) for resuming read operations
pub const LOW_WATERMARK_BYTES: usize = MAX_PTY_BUFFER_BYTES / 2;

/// Bounded PTY Stream Controller
pub struct PtyStreamController {
    buffer: Arc<Mutex<VecDeque<u8>>>,
    is_suspended: Arc<Mutex<bool>>,
}

impl Default for PtyStreamController {
    fn default() -> Self {
        Self::new()
    }
}

impl PtyStreamController {
    pub fn new() -> Self {
        Self {
            buffer: Arc::new(Mutex::new(VecDeque::with_capacity(MAX_PTY_BUFFER_BYTES))),
            is_suspended: Arc::new(Mutex::new(false)),
        }
    }

    /// Push PTY chunk into buffer, evaluating backpressure
    pub async fn push_chunk(&self, chunk: &[u8]) -> bool {
        let mut buf = self.buffer.lock().await;
        if buf.len() + chunk.len() > MAX_PTY_BUFFER_BYTES {
            warn!("PTY buffer saturation! Truncating runaway stream output");
            return false;
        }

        buf.extend(chunk);
        if buf.len() >= HIGH_WATERMARK_BYTES {
            let mut suspended = self.is_suspended.lock().await;
            if !*suspended {
                debug!(
                    "PTY Buffer crossed high watermark ({} bytes). Suspending PTY reads.",
                    buf.len()
                );
                *suspended = true;
            }
        }
        true
    }

    /// Pull bytes for transmission over QUIC stream, releasing backpressure if below low watermark
    pub async fn pull_chunk(&self, max_bytes: usize) -> Vec<u8> {
        let mut buf = self.buffer.lock().await;
        let drain_len = max_bytes.min(buf.len());
        let chunk: Vec<u8> = buf.drain(..drain_len).collect();

        if buf.len() <= LOW_WATERMARK_BYTES {
            let mut suspended = self.is_suspended.lock().await;
            if *suspended {
                debug!(
                    "PTY Buffer drained below low watermark ({} bytes). Resuming PTY reads.",
                    buf.len()
                );
                *suspended = false;
            }
        }

        chunk
    }

    /// Check if reader is currently suspended
    pub async fn is_suspended(&self) -> bool {
        *self.is_suspended.lock().await
    }

    /// Current buffered byte count
    pub async fn buffered_bytes(&self) -> usize {
        self.buffer.lock().await.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pty_backpressure_suspension_and_resume() {
        let controller = PtyStreamController::new();
        assert!(!controller.is_suspended().await);

        // Push 240 KB chunk (crosses high watermark 230.4 KB)
        let large_chunk = vec![0x41; 240 * 1024];
        assert!(controller.push_chunk(&large_chunk).await);
        assert!(controller.is_suspended().await);

        // Pull 150 KB chunk (drains below low watermark 128 KB)
        let pulled = controller.pull_chunk(150 * 1024).await;
        assert_eq!(pulled.len(), 150 * 1024);
        assert!(!controller.is_suspended().await);
    }
}
