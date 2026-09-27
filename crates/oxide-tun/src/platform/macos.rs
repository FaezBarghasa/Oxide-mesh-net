//! macOS utun implementation

use crate::{
    RawFd, TunConfig,
    error::{Result, TunError},
    platform::{AsyncFd, QueueStats, TunDevice, TunQueue},
};
use libc::{IFNAMSIZ, c_int, c_short, c_ulong, c_void};
use std::io;
use std::os::fd::{AsRawFd, FromRawFd, IntoRawFd, OwnedFd};

const TUN_DEV: &str = "/dev/utun";
const TUNSETIFF: c_ulong = 0x400454ca;
const IFF_TUN: c_short = 0x0001;
const IFF_NO_PI: c_short = 0x1000;

#[repr(C)]
struct IfReq {
    ifr_name: [c_char; IFNAMSIZ],
    ifr_flags: c_short,
}

type c_char = i8;

/// macOS TUN device using utun
pub struct MacosTunDevice {
    name: String,
    fd: OwnedFd,
    queue: MacosTunQueue,
}

impl MacosTunDevice {
    pub fn new(config: &TunConfig) -> Result<Self> {
        // Find available utun device
        let mut fd: Option<OwnedFd> = None;
        let mut name = String::new();

        for i in 0..16 {
            let path = format!("{}{}", TUN_DEV, i);
            match std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(&path)
            {
                Ok(file) => {
                    let mut ifr: IfReq = unsafe { std::mem::zeroed() };
                    ifr.ifr_flags = IFF_TUN | IFF_NO_PI;

                    let ret = unsafe { libc::ioctl(file.as_raw_fd(), TUNSETIFF, &ifr) };
                    if ret >= 0 {
                        let actual_name = std::str::from_utf8(&ifr.ifr_name)
                            .map_err(|_| TunError::Internal("Invalid interface name".into()))?
                            .trim_end_matches('\0');
                        name = actual_name.to_string();
                        fd = Some(file.into());
                        break;
                    }
                }
                Err(_) => continue,
            }
        }

        let fd = fd.ok_or_else(|| TunError::Internal("No available utun device".into()))?;
        let queue = MacosTunQueue::new(fd.try_clone().map_err(|e| TunError::Io(e))?)?;

        let device = Self { name, fd, queue };

        device.configure(config)?;
        Ok(device)
    }

    fn configure(&self, config: &TunConfig) -> Result<()> {
        self.set_mtu(config.mtu)?;
        if config.up {
            self.set_up(true)?;
        }
        Ok(())
    }
}

impl TunDevice for MacosTunDevice {
    fn name(&self) -> &str {
        &self.name
    }

    fn num_queues(&self) -> usize {
        1
    }

    fn queue(&self, index: usize) -> Option<Box<dyn TunQueue>> {
        if index == 0 {
            Some(Box::new(self.queue.clone()) as Box<dyn TunQueue>)
        } else {
            None
        }
    }

    fn queues(&self) -> Vec<Box<dyn TunQueue>> {
        vec![Box::new(self.queue.clone()) as Box<dyn TunQueue>]
    }

    fn set_mtu(&self, mtu: u16) -> Result<()> {
        // Use sysctl or ioctl
        Ok(())
    }

    fn set_up(&self, up: bool) -> Result<()> {
        // Use ifconfig or ioctl
        Ok(())
    }

    fn set_ipv4(&self, addr: std::net::Ipv4Addr, prefix_len: u8) -> Result<()> {
        Ok(())
    }

    fn set_ipv6(&self, addr: std::net::Ipv6Addr, prefix_len: u8) -> Result<()> {
        Ok(())
    }

    fn set_offload(&self, enable: bool) -> Result<()> {
        Ok(())
    }

    fn close(&self) -> Result<()> {
        Ok(())
    }
}

/// macOS TUN queue
#[derive(Clone)]
pub struct MacosTunQueue {
    inner: AsyncFd,
}

impl MacosTunQueue {
    pub fn new(fd: OwnedFd) -> Result<Self> {
        Ok(Self {
            inner: AsyncFd::new(fd.as_raw_fd(), 0),
        })
    }
}

impl TunQueue for MacosTunQueue {
    fn queue_id(&self) -> usize {
        0
    }

    fn as_raw_fd(&self) -> RawFd {
        self.inner.as_raw_fd()
    }

    fn set_nonblocking(&self, nonblocking: bool) -> Result<()> {
        self.inner.set_nonblocking(nonblocking)
    }

    fn stats(&self) -> QueueStats {
        self.inner.stats()
    }
}

impl AsyncRead for MacosTunQueue {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        AsyncRead::poll_read(std::pin::Pin::new(&mut self.get_mut().inner), cx, buf)
    }
}

impl AsyncWrite for MacosTunQueue {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<io::Result<usize>> {
        AsyncWrite::poll_write(std::pin::Pin::new(&mut self.get_mut().inner), cx, buf)
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        AsyncWrite::poll_flush(std::pin::Pin::new(&mut self.get_mut().inner), cx)
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        AsyncWrite::poll_shutdown(std::pin::Pin::new(&mut self.get_mut().inner), cx)
    }
}

pub fn create_tun_macos(config: &TunConfig) -> Result<Box<dyn TunDevice>> {
    let device = MacosTunDevice::new(config)?;
    Ok(Box::new(device))
}
