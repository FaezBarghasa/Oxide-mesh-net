# Configuration Reference Guide: Oxide Mesh Net

This document provides a comprehensive reference for all configuration options across `oxide-daemon`, `oxide-cli`, Anti-DPI profiles, and Iranian CDN Edge Bridges.

---

## 1. File Locations & Formats

The daemon accepts both **TOML** (`.toml`) and **JSON** (`.json`) configuration formats:

| Location | Purpose |
|---|---|
| `/etc/oxide/daemon.toml` | Default system-wide daemon configuration (recommended for systemd) |
| `./daemon.toml` | Local working directory configuration |
| `/run/oxide/daemon.sock` | Default IPC domain socket for non-root / root control plane |
| `/etc/oxide/anti-dpi.toml` | Anti-DPI & Censorship Evasion Profile |

---

## 2. Core Daemon Configuration Schema (`daemon.toml`)

```toml
# ==============================================================================
# Node Identity & Network Group
# ==============================================================================

# 32-byte hex-encoded cryptographic node ID (Ed25519 identity key)
node_id = "0000000000000000000000000000000000000000000000000000000000000001"

# Mesh cluster or organizational network name
mesh_name = "production-mesh"

# Optional Actix Web / MQTT coordinator URL
coordinator_url = "https://coord.oxide.mesh:8443"

# ==============================================================================
# Virtual Network Device (TUN)
# ==============================================================================

# TUN interface name on host OS
tun_name = "oxide0"

# Configured Maximum Transmission Unit (MTU)
# Standard: 1420 (Ethernet 1500 - QUIC/WireGuard outer headers)
# Cellular / Encapsulated: 1280 (IPv6 minimum)
mtu = 1420

# UDP transport listen port on host physical interface
listen_port = 51820

# ==============================================================================
# Addressing
# ==============================================================================

# Assigned overlay IPv4 address (CGNAT 100.64.0.0/10 block)
overlay_ipv4 = "100.64.0.42"

# Assigned overlay IPv6 address (Unique Local Address ULA fd00::/8)
overlay_ipv6 = "fd00:1d:e::42"

# ==============================================================================
# Autonomous Network Services
# ==============================================================================

# MagicDNS self-healing watchdog (maintains .oxide domain resolution)
enable_dns = true

# Dynamic TCP MSS clamping (RFC 1624 incremental checksum update)
# Automatically matches TCP MSS to active DPLPMTUD PMTU to prevent fragmentation
enable_mss_clamp = true

# Path to Unix IPC socket for CLI control
# Default: /run/oxide/daemon.sock or /tmp/oxide-daemon.sock
socket_path = "/run/oxide/daemon.sock"

# ==============================================================================
# Static Subnet Routes (Optional)
# ==============================================================================

[[static_routes]]
prefix = "192.168.10.0/24"
target_node = "gateway-node-id-hex"
pmtu = 1380
```

---

## 3. Virtual Host & Domain Reverse Proxy Configuration

Oxide Mesh Net routes domains and subdomains directly to local ports.

### CLI Management
```bash
# Add route:
oxide vhost sub.domain.com 8282

# Add second subdomain:
oxide vhost sub3.domain.com 6543

# Add wildcard route:
oxide vhost "*.internal.company" 9000
```

### How It Works:
1. Client connects via HTTP to port `80` (or mesh ingress).
2. The daemon performs an atomic **non-destructive socket peek** to inspect the `Host:` header without consuming the stream.
3. The connection is forwarded with zero-copy stream bridging to `127.0.0.1:<target_port>`.
4. If the hostname is unrecognized, a clean `404 Not Found` response is returned.

---

## 4. Anti-DPI & Censorship Evasion Profile (`anti-dpi-profile.toml`)

Designed for high-adversity middlebox environments (e.g. GFW, TIC/MCI/Irancell):

```toml
[transport.morphing]
enabled = true
header_magic_init      = 0xA4F2D81B
header_magic_response  = 0x7E3C9104
header_magic_data      = 0x5D82AA39
header_magic_keepalive = 0x12C0FFE8
junk_packet_count_min  = 2
junk_packet_count_max  = 5
handshake_padding_min  = 40
handshake_padding_max  = 280

[transport.tls_camouflage]
enabled = true
sni_target = "www.microsoft.com"
sni_port   = 443
server_public_key = "4f8a91bc7d2e3f4a0b5c6d7e8f90123456789abcdef0123456789abcdef01234"
short_ids = ["a1b2c3d4", "e5f60718"]

[transport.port_hopping]
enabled = true
interval_seconds = 180
ports = [51820, 52100, 53400, 54210, 58900]
```

---

## 5. Iranian CDN Edge Bridge Configuration (ArvanCloud / ParsPack)

When UDP is completely blocked or throttled by state firewalls, use the CDN Edge Tunnel:

```toml
[transport.cdn_bridge]
enabled = true
cdn_provider = "arvancloud" # "arvancloud" or "parspack"
edge_domain  = "mesh-edge.yourdomain.ir"
ws_path      = "/api/v1/stream"
use_tls      = true
port         = 443
```

This encapsulates mesh UDP frames into WebSocket frames over TLS (`wss://mesh-edge.yourdomain.ir/api/v1/stream`), bypassing UDP blocking while terminating at ArvanCloud or ParsPack reverse proxies.

---

## 6. Access Control Rules (`acl.json`)

Hot-reload zero-trust microsegmentation rules via `oxide acl reload --file rules.json`:

```json
[
  {
    "id": "allow-ssh-admin",
    "action": "allow",
    "direction": "both",
    "src_prefix": "100.64.0.0/24",
    "dst_prefix": "100.64.0.10/32",
    "protocol": 6,
    "dst_port_start": 22,
    "dst_port_end": 22
  },
  {
    "id": "allow-web-ingress",
    "action": "allow",
    "direction": "inbound",
    "src_prefix": "0.0.0.0/0",
    "dst_prefix": "100.64.0.42/32",
    "protocol": 6,
    "dst_port_start": 80,
    "dst_port_end": 443
  }
]
```
