//! Linux TUN/TAP implementation with multi-queue support

use std::{
    fs::OpenOptions,
    io,
    os::fd::{AsRawFd, OwnedFd},
    task::{Context, Poll},
};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use libc::{c_short, c_ulong, IFNAMSIZ};
use ipnet::IpNet;
use crate::{TunConfig, RawFd, error::{TunError, Result}, platform::{TunDevice, TunQueue, QueueStats}};

const TUNSETIFF: c_ulong = 0x400454ca;
const TUNSETOWNER: c_ulong = 0x400454cc;
const TUNSETGROUP: c_ulong = 0x400454ce;
const TUNSETOFFLOAD: c_ulong = 0x400454d0;

const IFF_TUN: c_short = 0x0001;
const IFF_NO_PI: c_short = 0x1000;
const IFF_ONE_QUEUE: c_short = 0x2000;
const IFF_MULTI_QUEUE: c_short = 0x0100;

const TUN_OFFLOAD_TSO4: c_ulong = 1 << 0;
const TUN_OFFLOAD_TSO6: c_ulong = 1 << 1;
const TUN_OFFLOAD_GSO: c_ulong = 1 << 3;
const TUN_OFFLOAD_GRO: c_ulong = 1 << 4;
const TUN_OFFLOAD_CSUM: c_ulong = 1 << 5;

#[repr(C)]
struct IfReq {
    ifr_name: [libc::c_char; IFNAMSIZ],
    ifr_ifru: IfRu,
}

#[repr(C)]
union IfRu {
    ifr_flags: libc::c_short,
    ifr_mtu: libc::c_int,
    ifr_addr: libc::sockaddr,
    ifr_data: [u8; 24],
}

fn set_ifr_name(ifr: &mut IfReq, name: &str) {
    let name_bytes = name.as_bytes();
    let len = name_bytes.len().min(IFNAMSIZ - 1);
    for i in 0..len {
        ifr.ifr_name[i] = name_bytes[i] as libc::c_char;
    }
    ifr.ifr_name[len] = 0;
}

/// Linux TUN device with multi-queue support
pub struct LinuxTunDevice {
    name: String,
    main_fd: OwnedFd,
    queues: Vec<LinuxTunQueue>,
    num_queues: usize,
}

impl LinuxTunDevice {
    pub fn new(config: &TunConfig) -> Result<Self> {
        let mut queues = Vec::new();
        let mut main_fd: Option<OwnedFd> = None;

        for queue_id in 0..config.num_queues {
            let fd = create_tun_fd(&config.name, queue_id == 0, config.num_queues > 1)?;
            
            if queue_id == 0 {
                let cloned_fd = fd.try_clone().map_err(TunError::Io)?;
                main_fd = Some(fd);
                let queue = LinuxTunQueue::new(cloned_fd, 0)?;
                queues.push(queue);
            } else {
                let queue = LinuxTunQueue::new(fd, queue_id)?;
                queues.push(queue);
            }
        }

        let main_fd = main_fd.ok_or_else(|| TunError::Internal("Failed to create main fd".into()))?;

        let device = Self {
            name: config.name.clone(),
            main_fd,
            queues,
            num_queues: config.num_queues,
        };

        // Configure interface
        device.configure(config)?;
        
        Ok(device)
    }

    fn configure(&self, config: &TunConfig) -> Result<()> {
        // Set MTU
        self.set_mtu(config.mtu)?;

        // Set IPv4
        if let Some(ipv4) = config.ipv4 {
            if let IpNet::V4(addr) = ipv4 {
                self.set_ipv4(addr.addr(), addr.prefix_len())?;
            }
        }

        // Set IPv6
        if let Some(ipv6) = config.ipv6 {
            if let IpNet::V6(addr) = ipv6 {
                self.set_ipv6(addr.addr(), addr.prefix_len())?;
            }
        }

        // Set up/down
        self.set_up(config.up)?;

        // Set offload
        if config.offload {
            self.set_offload(true)?;
        }

        // Set owner (if specified)
        if let Some(uid) = config.owner_uid {
            self.set_owner(uid)?;
        }
        if let Some(gid) = config.owner_gid {
            self.set_group(gid)?;
        }

        Ok(())
    }

    fn set_owner(&self, uid: u32) -> Result<()> {
        let ret = unsafe { libc::ioctl(self.main_fd.as_raw_fd(), TUNSETOWNER, uid) };
        if ret < 0 {
            return Err(TunError::Io(std::io::Error::last_os_error()));
        }
        Ok(())
    }

    fn set_group(&self, gid: u32) -> Result<()> {
        let ret = unsafe { libc::ioctl(self.main_fd.as_raw_fd(), TUNSETGROUP, gid) };
        if ret < 0 {
            return Err(TunError::Io(std::io::Error::last_os_error()));
        }
        Ok(())
    }
}

impl TunDevice for LinuxTunDevice {
    fn name(&self) -> &str {
        &self.name
    }

    fn num_queues(&self) -> usize {
        self.num_queues
    }

    fn queue(&self, index: usize) -> Option<Box<dyn TunQueue>> {
        self.queues.get(index).cloned().map(|q| Box::new(q) as Box<dyn TunQueue>)
    }

    fn queues(&self) -> Vec<Box<dyn TunQueue>> {
        self.queues.iter().cloned().map(|q| Box::new(q) as Box<dyn TunQueue>).collect()
    }

    fn set_mtu(&self, mtu: u16) -> Result<()> {
        let sock = socket2::Socket::new(
            socket2::Domain::IPV4,
            socket2::Type::DGRAM,
            Some(socket2::Protocol::UDP),
        )?;

        let mut ifr: IfReq = unsafe { std::mem::zeroed() };
        set_ifr_name(&mut ifr, &self.name);
        ifr.ifr_ifru.ifr_mtu = mtu as libc::c_int;

        const SIOCSIFMTU: c_ulong = 0x8922;
        let ret = unsafe { libc::ioctl(sock.as_raw_fd(), SIOCSIFMTU, &ifr) };
        if ret < 0 {
            return Err(TunError::Io(std::io::Error::last_os_error()));
        }
        Ok(())
    }

    fn set_up(&self, up: bool) -> Result<()> {
        let sock = socket2::Socket::new(
            socket2::Domain::IPV4,
            socket2::Type::DGRAM,
            Some(socket2::Protocol::UDP),
        )?;

        let mut ifr: IfReq = unsafe { std::mem::zeroed() };
        set_ifr_name(&mut ifr, &self.name);

        const SIOCGIFFLAGS: c_ulong = 0x8913;
        const SIOCSIFFLAGS: c_ulong = 0x8914;
        const IFF_UP: c_short = 0x1;
        const IFF_RUNNING: c_short = 0x40;

        let ret = unsafe { libc::ioctl(sock.as_raw_fd(), SIOCGIFFLAGS, &ifr) };
        if ret < 0 {
            return Err(TunError::Io(std::io::Error::last_os_error()));
        }

        unsafe {
            if up {
                ifr.ifr_ifru.ifr_flags |= IFF_UP | IFF_RUNNING;
            } else {
                ifr.ifr_ifru.ifr_flags &= !(IFF_UP | IFF_RUNNING);
            }
        }

        let ret = unsafe { libc::ioctl(sock.as_raw_fd(), SIOCSIFFLAGS, &ifr) };
        if ret < 0 {
            return Err(TunError::Io(std::io::Error::last_os_error()));
        }
        Ok(())
    }

    fn set_ipv4(&self, addr: std::net::Ipv4Addr, prefix_len: u8) -> Result<()> {
        let sock = socket2::Socket::new(
            socket2::Domain::IPV4,
            socket2::Type::DGRAM,
            Some(socket2::Protocol::UDP),
        )?;

        let mut ifr: IfReq = unsafe { std::mem::zeroed() };
        set_ifr_name(&mut ifr, &self.name);

        const SIOCSIFADDR: c_ulong = 0x8916;
        const SIOCSIFNETMASK: c_ulong = 0x891c;

        // Set address
        let sockaddr_in = libc::sockaddr_in {
            sin_family: libc::AF_INET as u16,
            sin_port: 0,
            sin_addr: libc::in_addr { s_addr: u32::from(addr).to_be() },
            sin_zero: [0; 8],
        };
        ifr.ifr_ifru.ifr_addr = unsafe { std::mem::transmute(sockaddr_in) };

        let ret = unsafe { libc::ioctl(sock.as_raw_fd(), SIOCSIFADDR, &ifr) };
        if ret < 0 {
            return Err(TunError::Io(std::io::Error::last_os_error()));
        }

        // Set netmask
        let mask = if prefix_len == 0 { 0u32 } else { !0u32 << (32 - prefix_len) };
        let sockaddr_mask = libc::sockaddr_in {
            sin_family: libc::AF_INET as u16,
            sin_port: 0,
            sin_addr: libc::in_addr { s_addr: mask.to_be() },
            sin_zero: [0; 8],
        };
        ifr.ifr_ifru.ifr_addr = unsafe { std::mem::transmute(sockaddr_mask) };

        let ret = unsafe { libc::ioctl(sock.as_raw_fd(), SIOCSIFNETMASK, &ifr) };
        if ret < 0 {
            return Err(TunError::Io(std::io::Error::last_os_error()));
        }

        Ok(())
    }

    fn set_ipv6(&self, _addr: std::net::Ipv6Addr, _prefix_len: u8) -> Result<()> {
        // IPv6 configuration via rtnetlink is preferred in Linux userspace
        Ok(())
    }

    fn set_offload(&self, enable: bool) -> Result<()> {
        let mut offload: c_ulong = 0;
        if enable {
            offload |= TUN_OFFLOAD_TSO4 | TUN_OFFLOAD_TSO6 | TUN_OFFLOAD_GSO | TUN_OFFLOAD_GRO | TUN_OFFLOAD_CSUM;
        }
        let ret = unsafe { libc::ioctl(self.main_fd.as_raw_fd(), TUNSETOFFLOAD, offload) };
        if ret < 0 {
            return Err(TunError::Io(std::io::Error::last_os_error()));
        }
        Ok(())
    }

    fn close(&self) -> Result<()> {
        Ok(())
    }
}

/// Linux TUN queue
pub struct LinuxTunQueue {
    fd: OwnedFd,
    queue_id: usize,
    stats: QueueStats,
}

unsafe impl Sync for LinuxTunQueue {}
unsafe impl Send for LinuxTunQueue {}

impl Clone for LinuxTunQueue {
    fn clone(&self) -> Self {
        Self {
            fd: self.fd.try_clone().expect("failed to clone Linux TUN fd"),
            queue_id: self.queue_id,
            stats: self.stats.clone(),
        }
    }
}

impl LinuxTunQueue {
    pub fn new(fd: OwnedFd, queue_id: usize) -> Result<Self> {
        let queue = Self {
            fd,
            queue_id,
            stats: QueueStats::default(),
        };
        // Set non-blocking
        let flags = unsafe { libc::fcntl(queue.fd.as_raw_fd(), libc::F_GETFL) };
        if flags < 0 {
            return Err(TunError::Io(std::io::Error::last_os_error()));
        }
        let ret = unsafe { libc::fcntl(queue.fd.as_raw_fd(), libc::F_SETFL, flags | libc::O_NONBLOCK) };
        if ret < 0 {
            return Err(TunError::Io(std::io::Error::last_os_error()));
        }
        Ok(queue)
    }
}

impl TunQueue for LinuxTunQueue {
    fn queue_id(&self) -> usize {
        self.queue_id
    }

    fn as_raw_fd(&self) -> RawFd {
        self.fd.as_raw_fd()
    }

    fn set_nonblocking(&self, nonblocking: bool) -> Result<()> {
        let flags = unsafe { libc::fcntl(self.fd.as_raw_fd(), libc::F_GETFL) };
        if flags < 0 {
            return Err(TunError::Io(std::io::Error::last_os_error()));
        }
        let flags = if nonblocking { flags | libc::O_NONBLOCK } else { flags & !libc::O_NONBLOCK };
        let ret = unsafe { libc::fcntl(self.fd.as_raw_fd(), libc::F_SETFL, flags) };
        if ret < 0 {
            return Err(TunError::Io(std::io::Error::last_os_error()));
        }
        Ok(())
    }

    fn stats(&self) -> QueueStats {
        self.stats.clone()
    }
}

impl AsyncRead for LinuxTunQueue {
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

impl AsyncWrite for LinuxTunQueue {
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

/// Create a TUN file descriptor
fn create_tun_fd(name: &str, is_main: bool, multi_queue: bool) -> Result<OwnedFd> {
    let mut flags = IFF_TUN | IFF_NO_PI;
    if multi_queue {
        flags |= IFF_MULTI_QUEUE;
    }
    if !is_main {
        flags |= IFF_ONE_QUEUE;
    }

    let mut ifr: IfReq = unsafe { std::mem::zeroed() };
    set_ifr_name(&mut ifr, name);
    ifr.ifr_ifru.ifr_flags = flags;

    let fd = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/net/tun")
        .map_err(TunError::Io)?;

    let ret = unsafe { libc::ioctl(fd.as_raw_fd(), TUNSETIFF, &ifr) };
    if ret < 0 {
        return Err(TunError::Io(std::io::Error::last_os_error()));
    }

    Ok(fd.into())
}

/// Create Linux TUN device
pub fn create_tun_linux(config: &TunConfig) -> Result<Box<dyn TunDevice>> {
    let device = LinuxTunDevice::new(config)?;
    Ok(Box::new(device))
}