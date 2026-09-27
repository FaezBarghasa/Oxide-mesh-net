# oxide-mesh-net: Product Requirements Document (PRD)

**Document Version:** 1.0.0  
**Status:** Approved  
**Author:** Oxide Networking Product & Systems Engineering Team  
**Scope:** Pure-Rust Zero-Trust Overlay Mesh Network

---

## 1. Executive Summary & Vision

`oxide-mesh-net` is an open-source, high-throughput, zero-trust overlay networking platform implemented natively in pure Rust. It connects servers, edge gateways, developer workstations, containers, and IoT/embedded nodes into an encrypted peer-to-peer mesh. By bypassing single-threaded userspace networking bottlenecks and central coordinator latency, `oxide-mesh-net` delivers wire-speed packet forwarding, resilient peer-to-peer hole punching, and continuous operation under adversarial and network-constrained environments.

---

## 2. Target Personas & Use Cases

### 2.1 Personas
- **DevOps & Infrastructure Engineers:** Securely interconnect multi-cloud clusters, hybrid on-premises environments, and bare-metal nodes with zero public IP exposure.
- **Embedded & IoT Systems Engineers:** Maintain persistent, low-overhead bi-directional telemetry and command channels across NATs and cellular connections.
- **Remote Developers & Engineering Teams:** Access internal microservices, private staging environments, and developer resources with automated split-DNS and single-sign-on (SSO/OIDC).
- **Security & Network Administrators:** Enforce granular L3/L4 microsegmentation and zero-trust access policies with cryptographic identity auditability.

### 2.2 Key Use Cases
1. **Multi-Cloud Private Interconnect:** Seamless L3 mesh connecting AWS, GCP, Hetzner, and bare-metal servers without complex IPsec or BGP tunneling.
2. **Adversarial & Censorship-Resistant Ingress:** Autonomous DPI evasion and fallback relaying through obfuscated QUIC/HTTPS tunnels.
3. **Local-First Distributed Edge Operations:** Nodes retain autonomous peer-to-peer routing and local state even during WAN disruptions or coordinator outages.
4. **Secure Developer Access & Services:** Frictionless peer discovery via MagicDNS (`<node>.oxide.mesh`) with zero configuration.

---

## 3. Functional Requirements (FR)

### 3.1 Identity, Authentication & Device Enrollment
- **FR-1.1:** Support non-interactive and interactive device enrollment using Ed25519 cryptographic keypairs.
- **FR-1.2:** Integrate OIDC/OAuth2 authentication (GitHub, Google, Keycloak) for human-driven machine registrations via Actix Web coordinator endpoints.
- **FR-1.3:** Provide pre-shared ephemeral node enrollment keys for automated CI/CD and container orchestration.
- **FR-1.4:** Support automated cryptographic key rotation without dropping active data-plane connections.

### 3.2 Transport & Data-Plane Forwarding
- **FR-2.1:** Implement virtual multi-queue network devices (TUN/TAP) on Linux, macOS, and Windows.
- **FR-2.2:** Support QUIC Unreliable Datagrams (RFC 9221) for zero-latency IP-in-QUIC packet encapsulation.
- **FR-2.3:** Provide multi-port UDP RSS striping to scale packet handling across multiple CPU cores.
- **FR-2.4:** Support reliable QUIC bidirectional streams on the same peer connection for control signaling, out-of-band management, and SSH tunneling.
- **FR-2.5:** Dynamically discover Path MTU (PMTU) and handle fragmentation seamlessly.

### 3.3 NAT Traversal & Relay Mesh
- **FR-3.1:** Implement STUN (RFC 5389) and ICE-like candidate gathering over IPv4 and IPv6.
- **FR-3.2:** Coordinate hole punching via the decentralized MQTT signaling plane.
- **FR-3.3:** Seamlessly fallback to encrypted DERP (Designated Encrypted Relay Protocol) relays when direct P2P connection fails.
- **FR-3.4:** Continuously measure peer latency and automatically migrate from relay to direct path when a viable route opens.

### 3.4 Routing, Subnet Gateways & Microsegmentation
- **FR-4.1:** Enable nodes to advertise physical subnets (Subnet Routers / Site-to-Site routing).
- **FR-4.2:** Support exit-node routing for tunneling all Internet traffic through a designated mesh peer.
- **FR-4.3:** Enforce declarative, zero-trust Access Control Lists (ACLs) matching source/dest IPs, ports, and cryptographic node tags.
- **FR-4.4:** Support live reload of ACL rules with sub-millisecond evaluation times.

### 3.5 Embedded Network Services
- **FR-5.1:** Provide built-in MagicDNS server answering PTR and A/AAAA queries for mesh nodes under `.oxide.mesh`.
- **FR-5.2:** Support Split-Horizon DNS upstream forwarding for corporate and public domains.
- **FR-5.3:** Provide secure file transfer (Oxide-Drop) over multiplexed QUIC streams.
- **FR-5.4:** Embed a secure terminal/SSH daemon authenticated via the peer's mesh identity key.

### 3.6 Management, UI & Observability
- **FR-6.1:** Unified CLI (`oxide-cli`) for full lifecycle node control, diagnostics, ping, and status querying.
- **FR-6.2:** Embedded Actix Web server serving a lightweight Dioxus WASM UI for desktop and web monitoring.
- **FR-6.3:** Structured JSON logging, OpenTelemetry tracing hooks, and Prometheus metrics export.

---

## 4. Non-Functional Requirements (NFR)

### 4.1 Performance & Throughput
- **NFR-1.1:** Data-plane forwarding latency overhead $\le 50\ \mu\text{s}$ compared to native kernel networking.
- **NFR-1.2:** Linear throughput scaling across available CPU cores utilizing multi-queue TUN and RSS UDP sockets.
- **NFR-1.3:** Memory footprint $\le 25\ \text{MB}$ resident set size (RSS) for idle background daemon.

### 4.2 Security & Cryptography
- **NFR-2.1:** Zero-trust architecture: every packet is authenticated and encrypted at the peer level.
- **NFR-2.2:** Cryptographic algorithms: Ed25519 (Identity), X25519 (Key Exchange), ChaCha20-Poly1305 and AES-256-GCM (AEAD), BLAKE3 / HKDF (KDF).
- **NFR-2.3:** Ephemeral session ratchets providing Perfect Forward Secrecy (PFS).
- **NFR-2.4:** Pure-Rust memory safety guarantee: `#![forbid(unsafe_code)]` in protocol parsing and core data-flow modules.

### 4.3 Reliability & Fault Tolerance
- **NFR-3.1:** 99.999% availability of peer-to-peer data plane once direct sessions are established.
- **NFR-3.2:** Offline-first architecture: node continues local subnet routing during coordinator disconnection.
- **NFR-3.3:** Seamless reconnection with exponential backoff and jitter upon network interface switching (e.g., Wi-Fi to 5G).

---

## 5. System Interfaces & Boundaries

```
                 ┌──────────────────────────────────────────────┐
                 │       oxide-coordinator (Actix Web API)      │
                 │   - Node Enrollment / Auth (OIDC / SSO)      │
                 │   - Network Map & Peer Discovery Authority   │
                 └──────────────────────┬───────────────────────┘
                                        │ (MQTT Signaling / TLS)
                                        ▼
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │                               oxide-daemon                                  │
 │                                                                             │
 │   ┌───────────────────────┐   ┌───────────────────────┐   ┌─────────────┐   │
 │   │  Multi-Queue TUN/TAP  │◄──┤  Packet Pipeline/ACL  ├──►│ QUIC Engine │   │
 │   └───────────────────────┘   └───────────────────────┘   └──────┬──────┘   │
 │                                                                  │          │
 └──────────────────────┬───────────────────────────────────────────┼──────────┘
                        │ (Local IPC / Actix Web)                   │ (UDP/P2P)
                        ▼                                           ▼
            ┌───────────────────────┐                   ┌───────────────────────┐
            │ oxide-cli / oxide-ui  │                   │   Remote Mesh Peer    │
            └───────────────────────┘                   └───────────────────────┘
```

---

## 6. Success Metrics & KPIs

| Metric | Target |
|---|---|
| Direct P2P Hole Punching Success Rate | $\ge 92\%$ across symmetric/cone NATs |
| Max Latency Overhead vs WireGuard | $\le 5\%$ additional latency |
| Max Throughput on 10GbE Interface | $\ge 8.5\ \text{Gbps}$ line rate |
| Zero-Trust ACL Lookup Duration | $\le 100\ \text{ns}$ per packet |
| Daemon Startup Time | $\le 200\ \text{ms}$ cold start |

---

## 7. Roadmap & Milestones

- **Phase 1 (Core Foundations):** Pure-Rust wire protocol, multi-queue TUN driver, QUIC datagram engine, crypto primitives.
- **Phase 2 (Signaling & NAT):** MQTT control plane integration, STUN/ICE hole punching, DERP relay fallback.
- **Phase 3 (Routing & Security):** Subnet routing, site-to-site gateway, zero-trust ACL microsegmentation engine.
- **Phase 4 (Services & Ecosystem):** MagicDNS resolver, Oxide-Drop, Oxide-SSH, embedded Dioxus WASM UI via Actix Web.
- **Phase 5 (Enterprise & Optimization):** Multi-tenant coordinator federation, quantum-resistant KEMs, SIMD packet acceleration.
