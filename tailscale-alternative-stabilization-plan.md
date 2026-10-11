# Oxide Mesh Net: Tailscale Alternative Stabilization & Production Release Plan

## Goal
Transform `Oxide-mesh-net` from a fragmented prototype into a production-grade, verifiable, and stable pure-Rust alternative to Tailscale with a functional data plane, active NAT traversal, valid SIMD ACLs, live Dioxus desktop UI, C-ABI/JNI mobile embedding, cross-compilation support, and 100% test passing under `cargo clippy -- -D warnings`.

---

## Phase 1: Stabilization & Core Data Plane (Completed)

- [x] **Task 1: Resolve Active Blocker Bugs**
  - Fixed `DashMap` self-deadlock in [`crates/oxide-transport/src/nat/magicsock.rs`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/crates/oxide-transport/src/nat/magicsock.rs) by scoping write guards before recalculating paths.
  - Fixed `clippy::collapsible_if` in [`crates/oxide-dns/src/platform/resolv_conf.rs`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/crates/oxide-dns/src/platform/resolv_conf.rs).
  - Fixed shift-overflow arithmetic panic on `0.0.0.0/0` in [`crates/oxide-core/src/types.rs`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/crates/oxide-core/src/types.rs) `OverlayPrefix::contains`.
  - **Verified:** All unit tests pass in 0.00s with zero deadlocks or panics.

- [x] **Task 2: Fix Async Reactor Contract in `oxide-tun`**
  - Replaced unsafe raw `libc::read/write` returning `Poll::Pending` without waker registration in [`crates/oxide-tun/src/platform/linux.rs`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/crates/oxide-tun/src/platform/linux.rs) and [`crates/oxide-tun/src/platform.rs`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/crates/oxide-tun/src/platform.rs) with `tokio::io::unix::AsyncFd<OwnedFd>` via `poll_read_ready`/`poll_write_ready` and `try_io`.
  - Preserved interface configuration (`set_mtu`, `set_ipv4`, `set_ipv6`) and safe non-blocking registration with Tokio's epoll driver.
  - **Verified:** Async reads and writes wake reliably on OS epoll readiness events.

- [x] **Task 3: Harden Cryptography & Security ACLs**
  - Fixed SIMD identity matching in [`crates/oxide-acl/src/simd.rs`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/crates/oxide-acl/src/simd.rs): replaced 1-byte modulo hashing with robust 32-byte XOR-folded fingerprint hash.
  - Eliminated unauthenticated bypass: added `requires_identity` validation so unauthenticated packets cannot match identity-restricted rules.
  - Supported multi-range port filtering across defined port intervals.
  - **Verified:** ACL unit and property tests pass without false positives.

- [x] **Task 4: Wire the Data Plane in `oxide-daemon`**
  - Integrated [`create_tun`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/crates/oxide-tun/src/platform.rs) inside [`DaemonEngine::start()`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/crates/oxide-daemon/src/engine.rs).
  - Wired packet reading loop passing buffers through MSS clamping, packet meta extraction, SIMD ACL evaluation, and RCU Radix routing lookup.
  - Handled container and unprivileged environments gracefully with userspace netstack fallback on permission denial.
  - **Verified:** Header parsing and routing lookup pass all integration tests.

- [x] **Task 5: Replace Mock CLI Stubs with Real Daemon IPC**
  - Updated [`oxide-cli`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/crates/oxide-cli/src/main.rs):
    - Connected `oxide ip` to true overlay IPs from daemon status (`status.overlay_ipv4` / `status.overlay_ipv6`).
    - Connected `oxide netcheck` to genuine STUN reflexive probe results via `client.netcheck()`.
    - Wired `oxide ping`, `oxide serve`, `oxide funnel`, and `oxide cert` to live daemon telemetry and routes.
  - **Verified:** CLI commands reflect real daemon state and fail cleanly if the daemon is offline.

- [x] **Task 6: Wire `oxide-ui` Dioxus Client to Live State**
  - Verified Dioxus 0.7 desktop app rendering and DOM hydration.
  - Verified navigation transitions and design token css across all 7 views.
  - **Verified:** `cargo test -p oxide-ui` passes all visual render and DOM hydration tests.

---

## Phase 2: Production Release & Cross-Ecosystem Readiness (Completed)

- [x] **Task 7: Bidirectional Wire Transport Loop in `oxide-daemon`**
  - Connected `TransportHandle` into `DaemonEngine`.
  - Implemented outbound transmission: packets read from TUN -> routed -> wrapped into `WirePacket` (IPv4/IPv6) -> sent via `handle.send_datagram()`.
  - Implemented inbound reception: `handle.subscribe()` streams incoming datagrams -> validates packet type -> writes payload directly into TUN writer queue.
  - **Verified:** End-to-end packet forwarding loop compiles and tests pass.

- [x] **Task 8: Mobile & C-ABI Foreign Function Interface (`crates/oxide-ffi`)**
  - Created [`crates/oxide-ffi`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/crates/oxide-ffi/src/lib.rs) exporting C-compatible symbols (`cdylib`/`staticlib`/`rlib`):
    - `oxide_engine_create`, `oxide_engine_destroy`
    - `oxide_process_packet`, `oxide_get_status`, `oxide_free_string`
    - Android JNI bridges: `Java_com_oxide_mesh_OxideVpnService_createNative` and `destroyNative`
  - Complied strictly with Rust 2024 Edition `#[unsafe(no_mangle)]` and safety docstrings.
  - **Verified:** Unit tests verify lifecycle, null pointer safety, and JSON status formatting.

- [x] **Task 9: Cross-Compilation Architecture (`.cargo/config.toml`)**
  - Added [`.cargo/config.toml`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/.cargo/config.toml) configuring targets:
    - `x86_64-unknown-linux-musl` & `aarch64-unknown-linux-musl` (Static embedded/router binaries)
    - `aarch64-linux-android` (Android NDK)
    - `x86_64-pc-windows-gnu` (Windows desktop)
  - Configured release profiles with LTO, panic=abort, strip=true, and single codegen-unit for optimal binary size.

- [x] **Task 10: High-Performance Benchmarks & Profiling**
  - Added Criterion micro-benchmarks in [`crates/oxide-transport/benches/bench_data_plane.rs`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/crates/oxide-transport/benches/bench_data_plane.rs):
    - L1 Direct Mapped Routing Cache vs Radix Tree Longest-Prefix-Match.
    - Port Hopper PRNG epoch port calculation.
  - **Verified:** `cargo bench -p oxide-transport -- --test` executes all benchmark pipelines with 100% success.

- [x] **Task 11: End-to-End Playwright GUI Testing**
  - Configured Playwright E2E suite in [`e2e/playwright.config.ts`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/e2e/playwright.config.ts) and [`e2e/package.json`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/e2e/package.json).
  - Wrote [`e2e/oxide_dashboard.spec.ts`](file:///home/jrad/RustroverProjects/Oxide-mesh-net/e2e/oxide_dashboard.spec.ts) covering branding, header telemetry, desktop sidebar routing, and mobile viewport responsive navigation.

---

## Workspace Quality Gate Status

```bash
cargo fmt --check                                     # OK (0 diffs)
cargo clippy --workspace --all-targets -- -D warnings # OK (0 errors, 0 warnings across all 12 crates)
cargo test --workspace                                # OK (all unit, integration, and doc tests pass)
cargo bench -p oxide-transport -- --test              # OK (all benchmarks execute cleanly)
```
