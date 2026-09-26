//! Platform abstraction for TUN interfaces

use std::io;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use crate::{TunConfig, RawFd, error::{TunError, Result}};

/// Platform-specific TUN device
pub trait TunDevice: Send + Sync {
    /// Get the interface name
    fn name(&self) -> &str;

    /// Get the number of queues
    fn num_queues(&self) -> usize;

    /// Get a queue by index
    fn queue(&self, index: usize) -> Option<Box<dyn TunQueue>>;

    /// Get all queues
    fn queues(&self) -> Vec<Box<dyn TunQueue>>;

    /// Set interface MTU
    fn set_mtu(&self, mtu: u16) -> Result<()>;

    /// Set interface up/down
    fn set_up(&self, up: bool) -> Result<()>;

    /// Set IPv4 address
    fn set_ipv4(&self, addr: std::net::Ipv4Addr, prefix_len: u8) -> Result<()>;

    /// Set IPv6 address
    fn set_ipv6(&self, addr: std::net::Ipv6Addr, prefix_len: u8) -> Result<()>;

    /// Enable/disable offload
    fn set_offload(&self, enable: bool) -> Result<()>;

    /// Close the device
    fn close(&self) -> Result<()>;
}

/// Platform-specific TUN queue
pub trait TunQueue: AsyncRead + AsyncWrite + Send + Sync + Unpin {
    /// Get queue ID
    fn queue_id(&self) -> usize;

    /// Get raw file descriptor
    fn as_raw_fd(&self) -> RawFd;

    /// Set non-blocking mode
    fn set_nonblocking(&self, nonblocking: bool) -> Result<()>;

    /// Get queue statistics
    fn stats(&self) -> QueueStats;
}

/// Queue statistics
#[derive(Debug, Clone, Default)]
pub struct QueueStats {
    pub packets_rx: u64,
    pub packets_tx: u64,
    pub bytes_rx: u64,
    pub bytes_tx: u64,
    pub errors_rx: u64,
    pub errors_tx: u64,
}

/// Create a new TUN device based on platform
pub fn create_tun(config: &TunConfig) -> Result<Box<dyn TunDevice>> {
    #[cfg(target_os = "linux")]
    {
        crate::platform::linux::create_tun_linux(config)
    }
    #[cfg(target_os = "windows")]
    {
        crate::platform::windows::create_tun_windows(config)
    }
    #[cfg(target_os = "macos")]
    {
        crate::platform::macos::create_tun_macos(config)
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        Err(TunError::PlatformNotSupported("Platform not supported".into()))
    }
}

/// Async wrapper for raw file descriptor
pub struct AsyncFd {
    fd: OwnedFd,
    queue_id: usize,
    stats: QueueStats,
}

unsafe impl Sync for AsyncFd {}

impl AsyncFd {
    pub fn new(fd: RawFd, queue_id: usize) -> Self {
        Self {
            fd: unsafe { OwnedFd::from_raw_fd(fd) },
            queue_id,
            stats: QueueStats::default(),
        }
    }
}

impl AsyncRead for AsyncFd {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let fd = self.fd.as_raw_fd();
        let buf_slice = buf.initialize_unfilled();
        
        let ret = unsafe { libc::read(fd, buf_slice.as_mut_ptr() as *mut _, buf_slice.len()) };
        if ret > 0 {
            buf.advance(ret as usize);
            self.stats.packets_rx += 1;
            self.stats.bytes_rx += ret as u64;
            Poll::Ready(Ok(()))
        } else if ret == 0 {
            Poll::Ready(Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF")))
        } else {
            let err = io::Error::last_os_error();
            if err.kind() == io::ErrorKind::WouldBlock {
                Poll::Pending
            } else {
                self.stats.errors_rx += 1;
                Poll::Ready(Err(err))
            }
        }
    }
}

impl AsyncWrite for AsyncFd {
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        let fd = self.fd.as_raw_fd();
        let ret = unsafe { libc::write(fd, buf.as_ptr() as *const _, buf.len()) };
        if ret > 0 {
            self.stats.packets_tx += 1;
            self.stats.bytes_tx += ret as u64;
            Poll::Ready(Ok(ret as usize))
        } else if ret == 0 {
            Poll::Ready(Err(io::Error::new(io::ErrorKind::WriteZero, "Write zero")))
        } else {
            let err = io::Error::last_os_error();
            if err.kind() == io::ErrorKind::WouldBlock {
                Poll::Pending
            } else {
                self.stats.errors_tx += 1;
                Poll::Ready(Err(err))
            }
        }
    }

    fn poll_flush(self: std::pin::Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: std::pin::Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

impl TunQueue for AsyncFd {
    fn queue_id(&self) -> usize {
        self.queue_id
    }

    fn as_raw_fd(&self) -> RawFd {
        self.fd.as_raw_fd()
    }

    fn set_nonblocking(&self, nonblocking: bool) -> Result<()> {
        #[cfg(target_os = "linux")]
        {
            use std::os::fd::AsRawFd;
            let flags = unsafe { libc::fcntl(self.fd.as_raw_fd(), libc::F_GETFL) };
            if flags < 0 {
                return Err(TunError::Io(io::Error::last_os_error()));
            }
            let flags = if nonblocking { flags | libc::O_NONBLOCK } else { flags & !libc::O_NONBLOCK };
            let ret = unsafe { libc::fcntl(self.fd.as_raw_fd(), libc::F_SETFL, flags) };
            if ret < 0 {
                return Err(TunError::Io(io::Error::last_os_error()));
            }
            Ok(())
        }
        #[cfg(not(target_os = "linux"))]
        {
            Err(TunError::PlatformNotSupported("Non-blocking not implemented for this platform".into()))
        }
    }

    fn stats(&self) -> QueueStats {
        self.stats.clone()
    }
}

// Platform-specific modules
#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub mod macos;