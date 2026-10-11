# Quickstart & Setup Guide: Oxide Mesh Net

This guide walks you through setting up and running **Oxide Mesh Net** in less than 2 minutes on Linux, macOS, and container environments.

---

## 1. Prerequisites

- **Operating System:** Linux (Kernel 5.10+, systemd, `cap_net_admin`), macOS 13+, or Windows 11.
- **Rust Toolchain:** Rust 1.85+ (Rust 2024 Edition) if building from source.
- **Privileges:** Root access (`sudo`) or `CAP_NET_ADMIN` + `CAP_NET_BIND_SERVICE` for TUN interface provisioning. (Rootless mode available via userspace SOCKS5).

---

## 2. Fast Build & Installation

```bash
# Clone the repository
git clone https://github.com/FaezBarghasa/Oxide-mesh-net.git
cd Oxide-mesh-net

# Compile optimized release binaries
cargo build --release

# Install binaries to system PATH
sudo install -m 755 target/release/oxide /usr/local/bin/oxide
sudo install -m 755 target/release/oxide-daemon /usr/local/bin/oxide-daemon
```

---

## 3. Fast Node Initialization (One-Liner)

Initialize node identity keys and configuration with `oxide init`:

```bash
# Initialize local configuration in current directory (daemon.toml)
oxide init

# Or initialize system-wide under /etc/oxide/
sudo mkdir -p /etc/oxide
sudo oxide init --config /etc/oxide/daemon.toml --mesh production
```

This generates cryptographic Ed25519 node identities, assigned overlay IPv4 (`100.64.0.x`) and IPv6 (`fd00:1d:e::x`) addresses, and ready-to-use TOML configuration.

---

## 4. Running the Background Daemon

### Option A: Systemd Service (Recommended for Production)

```bash
# 1. Install systemd service unit
sudo cp packaging/systemd/oxide-daemon.service /etc/systemd/system/

# 2. Ensure /etc/oxide directory exists with daemon.toml
sudo cp config/daemon.sample.toml /etc/oxide/daemon.toml

# 3. Enable and start daemon
sudo systemctl daemon-reload
sudo systemctl enable --now oxide-daemon

# 4. Verify status
sudo systemctl status oxide-daemon
```

### Option B: Interactive Foreground Running

```bash
# Run with administrative network capabilities
sudo oxide-daemon --config daemon.toml
```

### Option C: Rootless / Container Userspace Mode

If running in Docker, Kubernetes, or an unprivileged shell without `CAP_NET_ADMIN`:
```bash
# oxide-daemon automatically detects absence of root and boots Userspace Netstack:
oxide-daemon --no-dns

# SOCKS5 proxy will be available at:
# 127.0.0.1:1055 (HTTP/TCP & UDP ASSOCIATE supported)
```

---

## 5. Connecting and Verifying Mesh Status

```bash
# Check node health, PMTU, and connected peers
oxide status

# Query active mesh peers
oxide peers

# Run comprehensive STUN, NAT-type, and firewall path check
oxide netcheck

# Check local assigned overlay IPs
oxide ip
```

---

## 6. Virtual Host & Reverse Proxy Setup (Subdomain Routing)

Oxide Mesh Net includes built-in dynamic virtual host reverse proxying:

```bash
# Route sub.domain.com to local backend port 8282
oxide vhost sub.domain.com 8282

# Route sub3.domain.com to local backend port 6543
oxide vhost sub3.domain.com 6543

# Route wildcards to an internal microservice
oxide vhost "*.internal.net" 9000
```

Incoming HTTP requests to `sub.domain.com` are transparently routed to `127.0.0.1:8282` via non-destructive HTTP Host inspection.

---

## 7. Connecting to a Mesh Coordinator

To join a federated mesh cluster coordinated by an Actix Web server:

```bash
# Join coordinator network
oxide up --coordinator https://coord.oxide.mesh:8443

# Advertise as a subnet router
oxide up --advertise-routes 192.168.1.0/24

# Route all default gateway traffic via an exit node
oxide up --exit-node 100.64.0.1 --exit-node-allow-lan
```
