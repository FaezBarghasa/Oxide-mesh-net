# oxide-mesh-net

**Enterprise-Grade, High-Performance, Zero-Trust Overlay Mesh Network in Pure Rust**

[![Rust](https://img.shields.io/badge/Rust-2024_Edition-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Transport](https://img.shields.io/badge/Transport-QUIC_Datagrams-green.svg)](https://datatracker.ietf.org/doc/html/rfc9221)
[![HTTP Engine](https://img.shields.io/badge/HTTP_Engine-Actix_Web_4-purple.svg)](https://actix.rs/)

---

## 🌟 Executive Summary

`oxide-mesh-net` is a decentralized, memory-safe, hardware-aligned overlay mesh network engineered to supersede legacy userspace VPNs. Designed with a share-nothing architecture, it delivers wire-speed packet forwarding, zero-allocation serialization, and sub-millisecond control signaling across hybrid cloud, edge, and embedded deployments.

---

## 🚀 Key Architectural Features

- **Multi-Queue & Thread-per-Core Architecture:** Native multi-queue TUN/TAP virtual network device scaling with lockless per-core packet processing loops.
- **Hardware-Aligned Multi-Port RSS Striping:** Dynamically distributes UDP/QUIC datagrams across socket pools to fully exploit Network Interface Card (NIC) Receive Side Scaling.
- **Multiplexed QUIC Transport Plane:** Leverages QUIC Unreliable Datagrams (RFC 9221) for L3 IP transit alongside reliable multiplexed streams for signaling, file transfer, and remote terminal sessions on a single connection.
- **Decentralized Signaling via Embedded MQTT:** Sub-millisecond distributed state synchronization, ICE/STUN/DERP NAT traversal coordination, and ephemeral key rotation powered by pure-Rust MQTT (`rumqttc` / `rumqttd`).
- **Actix Web Standard:** Unified high-concurrency HTTP application surface across coordinator APIs, health probes, daemon administration, and embedded Dioxus WASM UI delivery.
- **Zero-Trust Identity & Cryptographic Agility:** Hierarchical Ed25519 device identities, X25519 ephemeral key agreement, ChaCha20-Poly1305 and AES-256-GCM AEAD encryption, and quantum-resistant KEM readiness.
- **Offline-First & Local-First Resilience:** Continuous mesh operation during coordinator partitions with local state caching backed by SurrealDB.

---

## 📦 Workspace Crates

| Crate | Description |
|---|---|
| [`oxide-core`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/crates/oxide-core) | Core data models, identity keys, network addresses, error definitions, and telemetry traits. |
| [`oxide-crypto`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/crates/oxide-crypto) | Cryptographic primitives, AEAD ciphers, key derivation (HKDF/Blake3), and session ratchets. |
| [`oxide-protocol`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/crates/oxide-protocol) | Binary wire framing envelopes, zero-copy serialization, and signaling message types. |
| [`oxide-tun`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/crates/oxide-tun) | Cross-platform multi-queue virtual network interface (TUN/TAP) driver. |
| [`oxide-transport`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/crates/oxide-transport) | QUIC connection pooling, datagram transit, RSS UDP striping, and relay fallback. |
| [`oxide-coordinator`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/crates/oxide-coordinator) | Central/distributed coordination server, OIDC enrollment, and network state authority using Actix Web. |
| [`oxide-daemon`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/crates/oxide-daemon) | Background node daemon managing data-plane packet loops, routing tables, and IPC surfaces. |
| [`oxide-dns`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/crates/oxide-dns) | MagicDNS resolver, Split-Horizon DNS interceptor, and overlay domain routing. |
| [`oxide-acl`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/crates/oxide-acl) | High-throughput zero-trust access control rules engine and microsegmentation packet filter. |
| [`oxide-cli`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/crates/oxide-cli) | Command-line interface for administration, diagnostics, and interactive mesh debugging. |
| [`oxide-ui`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/crates/oxide-ui) | Dioxus-based cross-platform dashboard compiled to WASM and served via Actix Web. |

---

## 🛠️ Quick Start

### Prerequisites

- Rust 1.85+ (2024 Edition)
- Linux (TUN device access, `cap_net_admin`), macOS, or Windows

### Building

```bash
# Build release binaries
cargo build --release

# Run tests across workspace
cargo test --workspace

# Linting and style verification
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --check
```

### Running a Node

```bash
# Initialize node keys and configuration
cargo run -p oxide-cli -- init

# Connect to the mesh
cargo run -p oxide-cli -- up --coordinator https://coord.oxide.mesh:8443
```

---

## 📚 Documentation Index

- [Architecture Master Blueprint](file:///home/jrad/RustroverProjects/Oxide-mesh-net/docs/architecture.md)
- [Product Requirements Document (PRD)](file:///home/jrad/RustroverProjects/Oxide-mesh-net/docs/prd.md)
- [System Design Specification](file:///home/jrad/RustroverProjects/Oxide-mesh-net/docs/design.md)
- [Test-Driven Development (TDD) Specification](file:///home/jrad/RustroverProjects/Oxide-mesh-net/docs/tdd.md)
- [DPI Circumvention & Obfuscation](file:///home/jrad/RustroverProjects/Oxide-mesh-net/docs/dpi-circumvention.md)
- [UI Architecture & Design Blueprint](file:///home/jrad/RustroverProjects/Oxide-mesh-net/docs/ui-blueprint.md)
- [Open Knowledge Format (OKF) Bundle](file:///home/jrad/RustroverProjects/Oxide-mesh-net/okf_bundle/SUMMARY.md)

---

## 📄 License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.
