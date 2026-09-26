# oxide-mesh-net: Frontend Architectural Blueprint & Client Interface Plan

**Author:** Office of the Chief Technology Officer  
**Document Class:** Client UI/UX Architecture & Engineering Execution Specification  
**Status:** Canonical  
**Target Ecosystem:** Pure-Rust Cross-Platform Client Interface (WebAssembly, Desktop, Mobile, Headless Server)  
**Primary Framework:** Dioxus (Strict Dependency Isolation, Zero External JavaScript Frameworks)  
**HTTP Distribution Standard:** Actix Web Exclusive (Embedded Daemon Delivery & Coordinator UI)

---

## Executive Vision & UI Architecture Principles

The frontend interface for `oxide-mesh-net` (`oxide-ui`) must deliver an instantaneous, reactive, and resource-conscious management surface across heterogeneous environments: headless edge servers, desktop operating systems (macOS, Linux, Windows), and mobile platforms (Android, iOS).

Where existing commercial VPN client applications suffer from multi-hundred-megabyte Electron runtimes, high idle memory consumption, and sluggish web-view wrappers, `oxide-ui` is designed according to the following foundational mandates:

1. **Pure-Rust Reactive Primitives:** Built exclusively using the Dioxus framework, compiling to highly optimized WebAssembly (WASM) for web and headless browser access, and binding directly to native platform web engines (`tao`/`wry`) for desktop targets.
2. **Sub-250 Kilobyte Distribution Target:** The web-delivered administration interface must compile to an aggregate payload under 250 KB (gzipped), served directly out of daemon memory via Actix Web with zero third-party CDN dependencies.
3. **Zero-Bloat Stylistic Footprint:** Complete prohibition of heavy Node.js or npm-based CSS build chains. The UI utilizes an embedded, hand-crafted, hardware-accelerated CSS engine leveraging system fonts, modern CSS grid/flexbox, and atomic design tokens to guarantee sub-millisecond initial layout rendering.
4. **Decoupled Client-Daemon Separation:** The UI operates strictly as an unprivileged presentation client, communicating with the privileged background engine (`oxide-daemon`) through an authenticated, non-blocking Inter-Process Communication (IPC) boundary (local Unix Domain Sockets or Windows Named Pipes) and local WebSocket connections managed by Actix Web.
5. **Specialized High-Adversity Controls:** Native, dedicated user interface surfaces for state-level Deep Packet Inspection (DPI) evasion, real-time handshake fragmentation tuning, Reality TLS target selection, and emergency transport failovers tailored for censored network environments.

---

## Hierarchical Interface Execution Matrix

Progress across developmental tiers is governed by architectural dependencies, platform validation, and protocol convergence rather than arbitrary calendar dates.

```
┌────────────────────────────────────────────────────────────────────────┐
│ TIER F-0: Foundational Client Architecture & WASM Optimization         │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
┌───────────────────────────────────▼────────────────────────────────────┐
│ TIER F-1: Core Lifecycle, Dynamic Peer Topology & Telemetry Engine     │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
┌───────────────────────────────────▼────────────────────────────────────┐
│ TIER F-2: Routing Fabric, Exit Node Selection & Subnet Governance      │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
┌───────────────────────────────────▼────────────────────────────────────┐
│ TIER F-3: DPI Circumvention, Anti-Censorship & Obfuscation Panel       │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
┌───────────────────────────────────▼────────────────────────────────────┐
│ TIER F-4: Zero-Trust Identity, Ephemeral Enrollment & ACL Builder      │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
┌───────────────────────────────────▼────────────────────────────────────┐
│ TIER F-5: Embedded Network Services Management Surfaces                │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
┌───────────────────────────────────▼────────────────────────────────────┐
│ TIER F-6: Native Platform Shells, System Trays & Mobile Adapters       │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
┌───────────────────────────────────▼────────────────────────────────────┐
│ TIER F-7: Cryptographic Auditing, Diagnostics & Forensic Tools         │
└────────────────────────────────────────────────────────────────────────┘
```

---

## Tier F-0: Foundational Client Architecture & WASM Optimization

### Task F-0.1: Core Compilation Pipeline & Binary Footprint Budgets

* **Architectural Scope:** Establish the compilation toolchain, Link-Time Optimization (LTO) pipelines, and binary size controls for `oxide-ui`.
* **Technical Specifics:**
  * Configure release profiles to enforce aggressive size minimization using `opt-level = "z"`, single codegen units, `panic = "abort"`, and complete symbol stripping.
  * Integrate binaryen post-processing (`wasm-opt -Oz`) into the core build pipeline to eliminate dead code branches, flatten AST structures, and compact variable encodings.
  * Implement automated Continuous Integration checks that fail the build if the uncompressed WASM binary exceeds 500 KB or if the Brotli/Gzip-compressed distribution bundle exceeds 200 KB.
  * Eliminate standard heavy JSON formatting libraries in client-facing hot paths, utilizing low-overhead binary deserialization (`postcard`) or compact deserializers to prevent binary bloat.
* **Failure Modes & Defenses:** Macro and template bloat from Dioxus component expansions are guarded against by isolating shared components into compact procedural functions and avoiding deeply nested closure captures.

### Task F-0.2: Local Daemon IPC Bridge & Actix Web Transport Layer

* **Architectural Scope:** Construct the reactive communication layer linking `oxide-ui` to `oxide-daemon`.
* **Technical Specifics:**
  * For desktop and mobile execution targets, implement a zero-cost local IPC protocol over Unix Domain Sockets (Linux/macOS) and Named Pipes (Windows), leveraging operating-system level access permissions to prevent unauthorized local processes from injecting configuration commands.
  * For headless server administration and browser management surfaces, route interface commands over an Actix-Web-powered WebSocket connection (`/api/v1/ws/control`).
  * Enforce bidirectional, reactive state streaming: the daemon continuously broadcasts binary delta frames representing network state shifts, peer availability updates, and packet throughput metrics.
  * Construct a resilient local reconnection state machine: if the background daemon restarts, the interface must enter an unobtrusive reconnecting state with exponential backoff and jitter, preserving local input focus without resetting unsubmitted forms.

### Task F-0.3: Zero-Bloat Design Token System & Hardware-Accelerated CSS

* **Architectural Scope:** Deliver an atomic, dependency-free visual presentation layer without third-party CSS frameworks.
* **Technical Specifics:**
  * Build a centralized CSS variable design token architecture defining semantic color spaces (surface, elevation, accent, interactive states, signal health indicators).
  * Design a dark-first aesthetic tailored for technical network operators, with contrast ratios exceeding WCAG AAA standards to ensure visibility across mobile screens in high ambient lighting.
  * Implement hardware-accelerated layouts utilizing CSS Grid and Flexbox exclusively, eliminating JavaScript-driven layout measurements.
  * Provide native responsive viewport adaptations: the interface must fluidly transition between a single-column layout for mobile/small window sizes and a multi-pane dashboard layout for desktop and wide browser sessions.

---

## Tier F-1: Core Lifecycle, Dynamic Peer Topology & Telemetry Engine

### Task F-1.1: Node State Machine & Main Telemetry Dashboard

* **Architectural Scope:** Build the primary command surface reflecting local node status, active interfaces, and overall mesh health.
* **Technical Specifics:**
  * Display high-level lifecycle toggles: a global, non-blocking Mesh Activation Switch (Connected, Disconnecting, Offline, Error).
  * Present instantaneous real-time metrics: cumulative ingress/egress bandwidth, packet-per-second (PPS) rates, current MTU, assigned overlay IPv4/IPv6 addresses, and active worker thread counts.
  * Implement high-frequency, low-overhead micro-sparklines rendered via SVG paths or an HTML5 Canvas context to display rolling 60-second throughput and latency trends without causing DOM re-render thrashing.
  * Integrate an immediate visual diagnostic indicator for intermediate NAT classification (e.g., Full-Cone, Restricted-Cone, Symmetric/Hard NAT) dynamically determined by the underlying discovery engine.

### Task F-1.2: Dynamic Peer Roster & Interactive Topology Map

* **Architectural Scope:** Engineer an intuitive, searchable peer directory paired with a spatial visualization of active mesh connections.
* **Technical Specifics:**
  * Implement a virtualized scrolling peer list capable of rendering thousands of concurrent nodes without performance degradation, supporting multi-criteria filtering (online status, operating system, subnet membership, tag assignments).
  * Display rich per-peer connection cards indicating direct P2P status (UDP hole-punched) versus relayed status (MASQUE relay), active UDP transmission ports, observed Round Trip Time (RTT), and cryptographic handshake freshness.
  * Build an interactive, purely declarative topology visualization: display nodes in a force-directed or circular layout illustrating logical relationships, active routes, and fallback relay paths.
  * Provide visual indicators for path transitions: when a peer migrates from a relayed path to a direct hole-punched UDP path, the connection vector must dynamically animate to reflect the optimized routing topology.

### Task F-1.3: Deep Node Inspector & Diagnostic Drawer

* **Architectural Scope:** Provide deep technical inspection capabilities for any selected peer node.
* **Technical Specifics:**
  * Implement a slide-out detailed inspection drawer presenting low-level transport metrics: negotiated QUIC version, congestion control algorithm state (BBR pacing rates, inflight bytes), and active Receive Side Scaling (RSS) socket pool assignments.
  * Expose an inline, non-blocking ICMP/QUIC path latency probe trigger allowing administrators to run instantaneous diagnostic pings and traceroutes directly within the mesh overlay.
  * Display the peer’s advertised cryptographic public keys, session re-key intervals, and hardware token provenance (e.g., TPM 2.0 endorsement status).

---

## Tier F-2: Routing Fabric, Exit Node Selection & Subnet Governance

### Task F-2.1: Exit Node Selector & Global Gateway Controller

* **Architectural Scope:** Build the interface for electing, monitoring, and managing default internet egress nodes.
* **Technical Specifics:**
  * Present a curated list of all peers advertising default gateway routes (`0.0.0.0/0`, `::/0`), sorted dynamically by measured latency and regional geography.
  * Deliver a single-click "Use as Exit Node" toggle with immediate visual confirmation of traffic redirection.
  * Implement an "Allow Local Network Access" configuration switch, providing clear visual warnings explaining how split-routing preserves local printer and LAN connectivity while the exit node is active.
  * Provide an active verification indicator: once an exit node is engaged, the UI triggers a background probe over the tunnel to verify public IP replacement and DNS leak protection, displaying the verified public egress IP to the user.

### Task F-2.2: Subnet Router Advertisement & Local Network Bridge Manager

* **Architectural Scope:** Enable administrators to expose and manage physical LAN segments through the node.
* **Technical Specifics:**
  * Construct a subnet advertising controller where operators can input local CIDR blocks (e.g., `192.168.1.0/24`) to bridge into the mesh.
  * Provide automated validation of input ranges: parse and highlight invalid subnet masks, broadcast address conflicts, or overlaps with reserved overlay address spaces.
  * Implement an approval and route conflict visualization: if multiple nodes advertise identical or overlapping subnets, the interface must display active leader/standby failover states and indicate which physical router currently owns the active VRRP-equivalent lease.

### Task F-2.3: Split-Horizon Routing & Per-Application Exclusion Configuration

* **Architectural Scope:** Allow granular control over which destinations traverse the overlay network versus local internet uplinks.
* **Technical Specifics:**
  * Provide a policy routing rule list where operators can specify custom domain patterns, specific IP ranges, or local application processes to bypass the mesh entirely.
  * Visualize routing decisions dynamically: provide a simple testing tool where users enter an IP address or domain name and the UI immediately renders the matching routing decision (Mesh TUN, Subnet Gateway, or Local Direct Egress).

---

## Tier F-3: DPI Circumvention, Anti-Censorship & Obfuscation Panel

### Task F-3.1: Protocol Morphing & Signature Scrambling Surface

* **Architectural Scope:** Configure and monitor low-level packet obfuscation designed to defeat stateful Deep Packet Inspection appliances.
* **Technical Specifics:**
  * Deliver an advanced "Stealth & Anti-DPI" settings panel containing granular controls for packet transformation.
  * Expose header magic scrambling parameters: allow operators to view and randomize 4-byte cryptographic header discriminators that strip standard WireGuard and QUIC initiation signatures.
  * Implement visual slider controls for handshake and payload padding boundaries (e.g., 40 to 280 bytes), explaining to the user how variable packet lengths invalidate statistical packet-size matching heuristics.
  * Provide an intuitive toggle for "Pre-Handshake Junk Injection": allow users to configure the burst count and payload range of random datagrams dispatched to confuse middlebox flow trackers.

### Task F-3.2: Reality TLS Camouflage & Active Probing Defense Configurator

* **Architectural Scope:** Configure TLS 1.3 Reality camouflage to bypass active network scanning clusters.
* **Technical Specifics:**
  * Provide an SNI Target selector allowing users to input or select high-reputation, unblocked cloud endpoints (e.g., `www.microsoft.com`, `gateway.icloud.com`).
  * Integrate an automatic SNI reachability tester: before applying an SNI target, the client verifies local reachability and confirms that the remote destination supports TLS 1.3 and ALPN HTTP/2 or HTTP/3.
  * Display the server public key and configured authentication short IDs (`short_id`), showing the active status of probe diversion mechanisms.
  * Provide an active scanner probe counter: display the number of unauthorized external probes that were successfully diverted to the authentic fallback target without revealing the mesh node’s existence.

### Task F-3.3: ClientHello Fragmentation & Delay Tuner

* **Architectural Scope:** Manage TCP-level packet fragmentation parameters to defeat shallow SNI filtering appliances.
* **Technical Specifics:**
  * Provide a ClientHello SNI split configuration tool: expose controls to adjust the byte offset where the initial TCP segment boundary is cleaved.
  * Implement a microscopic transmission delay slider (`split_delay_ms`, 1 to 20 milliseconds), providing clear technical tooltips explaining how line-rate middleboxes drop unbuffered out-of-order segments while destination servers reassemble them cleanly.
  * Display a live fragmentation status indicator confirming whether outgoing TCP ClientHello frames are actively being split across segment boundaries.

### Task F-3.4: Dynamic Port Hopping & Emergency Transport Failover Controller

* **Architectural Scope:** Monitor and govern dynamic port hopping and automatic transition to TCP fallback tunnels.
* **Technical Specifics:**
  * Expose port hopping range parameters (`port_range_start` to `port_range_end`) and hop interval sliders (e.g., 15 to 120 seconds).
  * Render an animated port-hopping timeline: visually depict the current active UDP port, the countdown until the next synchronized pseudorandom hop, and the upcoming port candidate.
  * Build an "Emergency Transport Mode" dashboard card: display the real-time health of the primary UDP transport plane. If UDP packet delivery drops below operational thresholds for longer than the failure window, prominently display that the system has transitioned to the Actix-Web-powered WebSocket Secure (WSS) tunnel on port 443.
  * Allow manual overrides: provide an "Enforce HTTPS/WSS Only" switch for environments where all UDP traffic is completely throttled or dropped by national firewalls.

---

## Tier F-4: Zero-Trust Identity, Ephemeral Enrollment & ACL Builder

### Task F-4.1: Authentication Lifecycles & Ephemeral Device Enrollment

* **Architectural Scope:** Manage the user authentication journey, OIDC federation, and device onboarding.
* **Technical Specifics:**
  * Deliver a clean, distraction-free authentication screen supporting OIDC redirect handshakes and browser-based OAuth2 flows via the coordinator’s Actix Web endpoints.
  * Implement the RFC 8628 Device Authorization Grant flow for headless servers and limited-input devices: display a clearly formatted verification URI along with an 8-character user code and a dynamically generated QR code for fast mobile scanning.
  * Support air-gapped pre-shared key enrollment: provide a secure input field for manual provisioning tokens in isolated networks where external identity providers are unreachable.
  * Display session validity counters and token expiration alerts, prompting users to re-authenticate seamlessly before active authorizations expire.

### Task F-4.2: Visual Zero-Trust ACL Policy Builder & Matrix Inspector

* **Architectural Scope:** Provide an intuitive interface for inspecting and designing microsegmentation rules.
* **Technical Specifics:**
  * Construct a two-dimensional Access Control Matrix: visualize identity tags (e.g., `tag:prod`, `tag:dev`, `tag:monitoring`) on both axes, with intersect cells indicating whether traffic is Allowed, Denied, or Conditionally Filtered.
  * Build a declarative rule editor for administrators: allow creation of fine-grained policies specifying source tags, destination tags, protocol bitmasks, and allowed port ranges (e.g., TCP 443, UDP 53).
  * Deliver an instant Policy Simulator: allow operators to select two arbitrary peers or tags and simulate a connection attempt across a specific port, instantly rendering whether the packet would be permitted or dropped by the node’s SIMD-accelerated ACL engine.

### Task F-4.3: Hardware Root-of-Trust & Cryptographic Certificate Viewer

* **Architectural Scope:** Expose platform security capabilities and hardware identity status.
* **Technical Specifics:**
  * Display a dedicated security status panel showing whether the node’s root private key is bound to a physical hardware security module (TPM 2.0 on Windows/Linux, Apple Secure Enclave on macOS, Android Keystore, or iOS Secure Enclave).
  * Provide an inspectable view of the node’s active certificate chain, showing cryptographic algorithms in use (e.g., Ed25519, ML-KEM/Kyber hybrid schemes), serial numbers, expiration timestamps, and coordinator trust bundle signatures.

---

## Tier F-5: Embedded Network Services Management Surfaces

### Task F-5.1: MagicDNS Controller & Split-Horizon Name Inspector

* **Architectural Scope:** Manage internal mesh domain names and upstream DNS configurations.
* **Technical Specifics:**
  * Display the node’s fully qualified MagicDNS domain name (e.g., `node-01.mesh.oxide`) with a single-click copy button and local name resolution status.
  * Provide an interface for configuring custom search domains, dynamic node aliases, and internal split-horizon routing domains (e.g., routing `*.corp.internal` to designated subnet resolvers).
  * Expose an upstream resolver configuration table: allow operators to select secure upstream DNS-over-HTTPS (DoH) providers with direct-IP bootstraps (`1.1.1.1`, `8.8.8.8`) to prevent DNS hijacking.
  * Include an in-browser DNS query diagnostic tool: test name resolution against the local embedded DNS engine directly from the UI, rendering query latency, resolved IP records, and upstream authority details.

### Task F-5.2: Oxide-Drop Peer-to-Peer Transfer Manager

* **Architectural Scope:** Build the user interface for multiplexed QUIC out-of-band file transfers.
* **Technical Specifics:**
  * Implement an intuitive drag-and-drop file staging area: dragging a file over an active peer card immediately stages an Oxide-Drop transmission.
  * Display an incoming transfer notification banner: show incoming file name, total payload size, sender cryptographic identity, and an explicit "Accept / Reject" prompt before any disk allocation occurs.
  * Render real-time transfer progress cards showing transfer speed, estimated time to completion, and live BLAKE3 block-level cryptographic verification status.
  * Provide a historical transfer log: display recently completed transfers with options to open the target folder or clear the transfer ledger.

### Task F-5.3: Oxide-SSH Integrated Web Terminal & Session Launcher

* **Architectural Scope:** Provide zero-configuration, browser-based secure terminal access to authorized mesh peers.
* **Technical Specifics:**
  * Embed an ultra-lightweight, hardware-accelerated terminal emulator component bound directly to an ephemeral SSH session over the mesh.
  * When initiating a session to an authorized peer, the UI requests an ephemeral terminal capability token from the coordinator and opens an authenticated multiplexed stream directly to the target’s embedded SSH micro-daemon.
  * Support full PTY terminal resizing, standard ANSI color palettes, clipboard copy/paste, and mobile-friendly soft keyboards with dedicated modifier keys (Ctrl, Alt, Esc, Tab).
  * Display active session audit metadata in the terminal header: remote peer identity, cryptographic cipher suite, session duration, and recording status.

### Task F-5.4: Oxide-Serve & Oxide-Funnel Service Ingress Configurator

* **Architectural Scope:** Allow users to expose local ports securely to mesh peers or the public internet using Actix Web reverse proxies.
* **Technical Specifics:**
  * Deliver an "Exposed Services" manager: allow operators to bind local loopback services (e.g., `127.0.0.1:8080`) to their internal MagicDNS name (Oxide-Serve).
  * Provide an "Expose to Public Internet" toggle (Oxide-Funnel): with a single click, route public HTTP/HTTPS traffic through external Actix Web edge relays.
  * Display automated TLS certificate provisioning status (Let's Encrypt / ACME challenge states) and render the active public URL.
  * Include a real-time request log showing incoming reverse-proxy requests, response status codes, and bandwidth utilization.

---

## Tier F-6: Native Platform Shells, System Trays & Mobile Adapters

### Task F-6.1: Desktop Native Tray Shell (macOS, Windows, Linux)

* **Architectural Scope:** Package `oxide-ui` into a non-intrusive native desktop system tray application.
* **Technical Specifics:**
  * Compile Dioxus targeting native desktop shells utilizing OS-native webviews (`wry`), maintaining an idle memory footprint under 30 MB.
  * Implement a native system tray icon reflecting connection status via dynamic color states (Connected, Reconnecting, Disconnected, Emergency Obfuscation Mode).
  * Provide an instant-access tray menu: fast toggles for Disconnect/Connect, Exit Node quick-picker, active peer count, and a direct link to open the full dashboard window.
  * Integrate native OS desktop notifications: alert users when peers share files via Oxide-Drop, when exit nodes fail over, or when network censorship forces transport fallback.

### Task F-6.2: Mobile Touch-First Interface & Power-Saving Shell (Android & iOS)

* **Architectural Scope:** Optimize the client experience for touch interactions, compact screens, and aggressive mobile battery constraints.
* **Technical Specifics:**
  * Deliver a bottom-navigation tabbed layout (Status, Peers, Routing, Services, Settings) with thumb-accessible interactive zones.
  * Implement adaptive frame-rate controls: throttle UI animations to 30 FPS or halt continuous telemetry polling when the mobile application is backgrounded or when the OS signals low-power mode.
  * Hook into mobile VPN status APIs: display native system VPN notification icons and handle OS sleep/wake transitions gracefully without showing spurious disconnection warnings.
  * Provide biometric authentication hooks (Face ID, Touch ID, Android Biometrics) to unlock the UI or approve sensitive operations (e.g., joining an administrative subnet or accepting SSH sessions).

### Task F-6.3: Headless Server Administration Shell (Actix Web Distribution)

* **Architectural Scope:** Deliver frictionless web administration for headless machines, cloud instances, and home routers.
* **Technical Specifics:**
  * The embedded Actix Web server inside `oxide-daemon` serves the pre-compressed Dioxus WASM binary directly from memory on a designated local port (e.g., `http://127.0.0.1:9090`).
  * Implement HTTP caching headers (`Cache-Control: public, max-age=31536000`, `ETag`) so the browser fetches the WASM binary only once, verifying integrity via sub-millisecond 304 Not Modified handshakes on subsequent loads.
  * Secure the web administration surface using local session tokens or mandatory HTTP basic/bearer authentication to prevent unauthorized local network access.
  * Ensure full functional parity: any operation available in the desktop tray app must be completely accessible via a standard web browser connecting to the headless server.

---

## Tier F-7: Cryptographic Auditing, Diagnostics & Forensic Tools

### Task F-7.1: Merkle-Tree Ledger & Cryptographic Audit Explorer

* **Architectural Scope:** Provide operators with transparent verification of all network and security state mutations.
* **Technical Specifics:**
  * Build an audit trail viewer that renders the node’s locally cached cryptographic event ledger.
  * Display every security-relevant action (peer enrollment, route change, ACL update, SSH session initiation) alongside the cryptographic signature of the initiating identity.
  * Implement an inline Merkle-proof validator: allow administrators to click any log entry and verify its cryptographic inclusion proof against the coordinator’s globally published root hash.

### Task F-7.2: Real-Time Packet Stream & Filter Debugger

* **Architectural Scope:** Deliver an integrated packet inspection and filtering diagnostic tool for network troubleshooting.
* **Technical Specifics:**
  * Implement a lightweight, non-blocking packet stream viewer: capture and display packet headers traversing the local virtual network interface.
  * Allow operators to apply live display filters by IP, protocol, port, or matching ACL rule.
  * Provide visual drop reasons: if a packet is dropped, clearly highlight the exact ACL bitmask, routing table mismatch, or anti-replay check responsible for the drop.

### Task F-7.3: Operational Verification & Frontend Readiness Checklist

To deem `oxide-ui` fully engineered and ready for general production deployment, the client implementation must achieve verified compliance across all operational benchmarks:

1. **Payload & Performance Benchmarks:**
   * Compressed WASM distribution bundle (HTML + CSS + WASM) must not exceed 250 KB over the wire.
   * Time to Interactive (TTI) on a standard mobile browser over a throttled 3G connection must occur in under 800 milliseconds.
   * Interface animation frame rates must maintain a consistent 60 FPS on desktop and at least 30 FPS on low-power mobile devices.
2. **Resource Footprint:**
   * Desktop system tray idle memory footprint must remain under 30 MB RSS.
   * Background CPU utilization of the UI client when idling must measure 0.0% across all operating systems.
3. **Resilience & State Convergence:**
   * When the background daemon is restarted or encounters network loss, the UI must automatically re-establish communication and synchronize state in under 500 milliseconds without losing user input focus.
4. **Architectural Purity:**
   * Absolute enforcement of pure-Rust Dioxus implementation with zero external JavaScript framework dependencies.
   * Strict adherence to Actix Web as the exclusive HTTP application server for headless asset distribution and WebSocket telemetry streaming.
