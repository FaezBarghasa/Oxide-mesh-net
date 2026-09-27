# oxide-mesh-net: Test-Driven Development (TDD) & Quality Assurance Specification

**Document Version:** 1.0.0  
**Status:** Approved  
**Author:** Oxide Networking QA & Verification Team  
**Standards:** Rust 2024 Edition, `cargo nextest`, `proptest`, `criterion`, `actix-web::test`

---

## 1. Test-Driven Development Philosophy

The `oxide-mesh-net` testing architecture enforces a rigorous, multi-tiered verification pipeline designed to guarantee zero-regression, memory safety, constant-time cryptographic operations, and high-throughput stability across all supported platforms.

```
+-------------------------------------------------------------------------+
| Level 5: System & Network Simulation (Virtual Network Namespaces / HIL) |
+-------------------------------------------------------------------------+
                                    ^
+-------------------------------------------------------------------------+
| Level 4: End-to-End API & Integration Tests (actix-web::test & IPC)    |
+-------------------------------------------------------------------------+
                                    ^
+-------------------------------------------------------------------------+
| Level 3: Property-Based & Fuzz Testing (proptest, arbitrary, cargo-fuzz)|
+-------------------------------------------------------------------------+
                                    ^
+-------------------------------------------------------------------------+
| Level 2: Concurrency & UB Sanitizers (Loom, Miri, ThreadSanitizer)      |
+-------------------------------------------------------------------------+
                                    ^
+-------------------------------------------------------------------------+
| Level 1: Unit & Component Isolation Tests (cargo test / cargo nextest)  |
+-------------------------------------------------------------------------+
```

---

## 2. Unit Testing Matrix by Crate

| Crate | Primary Focus | Key Unit Tests |
|---|---|---|
| [`oxide-core`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/crates/oxide-core) | Types, Addresses, Serialization | Node ID parsing, IPv4/IPv6 overlay address validation, Error display & downcasting. |
| [`oxide-crypto`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/crates/oxide-crypto) | Ciphers, Ratchets, Key Derivations | RFC 8439 ChaCha20-Poly1305 test vectors, HKDF expansion, Blake3 hashing, Constant-time equality. |
| [`oxide-protocol`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/crates/oxide-protocol) | Wire Framing, Parsing | Header alignment, Magic byte validation, Boundary truncation handling, Enum discriminators. |
| [`oxide-tun`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/crates/oxide-tun) | Virtual Interface IO | Multi-queue allocation, Non-blocking read/write timeouts, MTU reconfiguration. |
| [`oxide-transport`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/crates/oxide-transport) | QUIC Datagrams, RSS Striping | Socket striping hash distribution, Connection pooling, Datagram buffer fragmentation. |
| [`oxide-coordinator`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/crates/oxide-coordinator) | Actix Web Endpoints, Auth | Enrollment token validation, OIDC claim parsing, Network map generation, Telemetry ingestion. |
| [`oxide-daemon`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/crates/oxide-daemon) | Routing, State Machine | Node state transitions (Init -> Syncing -> Connected), Routing table updates. |
| [`oxide-dns`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/crates/oxide-dns) | DNS Server, Forwarding | `.oxide.mesh` A/AAAA record resolution, Upstream timeout fallbacks, Split-horizon rules. |
| [`oxide-acl`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/crates/oxide-acl) | Packet Filter, Microsegmentation | Radix tree CIDR matching, Port range parsing, Tag inheritance, Drop action verification. |

---

## 3. Property-Based Testing (`proptest`)

Property-based testing is required for all data parsers, serializers, routing trees, and ACL rule engines to eliminate edge-case vulnerabilities.

### 3.1 Wire Envelope Invariant Properties
- **Roundtrip Serialization:** For any arbitrary valid `PacketEnvelope`, `deserialize(serialize(packet)) == packet`.
- **Parser Panic-Free Guarantee:** For any random arbitrary byte sequence of length $0 \le L \le 65535$, parsing must return `Err(ProtocolError)` and never panic or abort.
- **Header Boundary Invariants:** Altering any single byte in the 128-bit authentication tag must fail decryption and authentication with 100% certainty.

```rust
use proptest::prelude::*;
use oxide_protocol::{PacketEnvelope, MessageType};

proptest! {
    #[test]
    fn test_wire_envelope_roundtrip(
        session_id in any::<u32>(),
        seq_num in any::<u64>(),
        payload in proptest::collection::vec(any::<u8>(), 0..1500)
    ) {
        let envelope = PacketEnvelope::new(MessageType::DataIpv4, session_id, seq_num, payload.clone());
        let encoded = envelope.to_bytes().expect("encoding must succeed");
        let decoded = PacketEnvelope::from_bytes(&encoded).expect("decoding must succeed");
        prop_assert_eq!(envelope, decoded);
    }
}
```

---

## 4. Integration Testing with `actix-web::test`

All HTTP API endpoints on `oxide-coordinator` and `oxide-daemon` must be verified using the in-memory `actix_web::test` runner without requiring live network port bindings.

```rust
#[cfg(test)]
mod tests {
    use actix_web::{test, App, http::StatusCode};
    use oxide_coordinator::routes::init_routes;
    use oxide_coordinator::state::CoordinatorState;

    #[actix_web::test]
    async fn test_enrollment_endpoint() {
        let state = CoordinatorState::new_mock();
        let app = test::init_service(
            App::new()
                .app_data(state.clone())
                .configure(init_routes)
        ).await;

        let req = test::TestRequest::post()
            .uri("/api/v1/auth/enroll")
            .set_json(&mock_enroll_payload())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
```

---

## 5. End-to-End Network Namespace Testing

Linux network namespaces (`ip netns`) are used in integration test suites to simulate full multi-node mesh topologies with virtual NAT routers on a single host.

### 5.1 Simulated Test Topologies
1. **Direct P2P Scenario:** Node A and Node B in separate namespaces with direct IP routing. Tests hole-punching and wire-speed datagram transit.
2. **Symmetric NAT Scenario:** Node A and Node B behind simulated Linux `iptables` symmetric NAT gateways. Tests automatic fallback to DERP relay.
3. **Subnet Router Scenario:** Node A acting as a subnet gateway for `192.168.10.0/24`. Tests IP forwarding, NAT masquerading, and ACL enforcement.

```bash
# Automated namespace test execution
sudo ./scripts/run_netns_tests.sh
```

---

## 6. Performance & Micro-Benchmarking (`criterion`)

All critical paths have strict latency and throughput budgets enforced via Criterion benchmarks:

| Benchmark Target | Method | Performance Threshold |
|---|---|---|
| Wire Envelope Serialization | `oxide_protocol::serialize` | $\le 45\ \text{ns} / \text{packet}$ |
| Wire Envelope Deserialization | `oxide_protocol::deserialize` | $\le 30\ \text{ns} / \text{packet}$ |
| ChaCha20-Poly1305 AEAD | `oxide_crypto::encrypt_packet` | $\ge 2.5\ \text{GB/s per core}$ |
| ACL Radix Lookup | `oxide_acl::evaluate_packet` | $\le 95\ \text{ns} / \text{lookup}$ |
| MagicDNS Internal Query | `oxide_dns::resolve_mesh_domain` | $\le 150\ \mu\text{s} / \text{query}$ |

```bash
# Run all workspace benchmarks
cargo bench --workspace
```

---

## 7. Fuzzing & Security Verification

- **Parser Fuzzing:** Continuous fuzz testing of all wire message parsers using `cargo fuzz` with LLVM `libFuzzer`.
- **Miri UB Verification:** Running `cargo miri test` over `oxide-crypto` and `oxide-protocol` to detect undefined behavior, memory leaks, and pointer aliasing violations.
- **Sanitizers:** Running CI builds with AddressSanitizer (ASan) and ThreadSanitizer (TSan) enabled:
  ```bash
  RUSTFLAGS="-Z sanitizer=address" cargo test --target x86_64-unknown-linux-gnu
  ```
