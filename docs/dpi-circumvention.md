# oxide-mesh-net: DPI Circumvention & Threat Model Guide

This guide details the technical mechanisms deployed in `oxide-mesh-net` to circumvent high-adversity Deep Packet Inspection (DPI) environments, such as those implemented by the Telecommunication Infrastructure Company (TIC).

## 1. Censorship Threat Vectors & Engineering Defenses

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                          INCOMING PACKET PIPELINE                               │
└──────────────────────────────────────┬──────────────────────────────────────────┘
                                       │
                ┌──────────────────────┴──────────────────────┐
                ▼                                             ▼
     [Layer 4 UDP / QUIC Path]                    [Layer 7 TCP Fallback Path]
                │                                             │
   ┌────────────┴────────────┐                  ┌─────────────┴─────────────┐
   ▼                         ▼                  ▼                           ▼
[Signature Morphing]   [Port Hopping]     [ClientHello Split]      [Actix WSS Tunnel]
 - Strips static WG     - Rotates UDP      - Fragments SNI          - Camouflaged as
   magic bytes            every 30s          across segments          legitimate web
 - Adds dynamic         - Evades flow-     - Defeats shallow          traffic on
   packet padding         based clamps       packet inspection        standard port 443
```

### Threat 1: Protocol Fingerprinting & Fixed Packet Lengths

* **Mechanism:** DPI appliances inspect the first few bytes of UDP packets for protocol identifiers (e.g., WireGuard's `0x01` handshake initiation) and calculate statistical packet-size signatures (e.g., standard WireGuard handshakes are always 148 bytes).

* **Oxide Defense:**

  * **Header Scrambling:** In `oxide-transport`, static header bytes are substituted with pre-shared cryptographic discriminators defined in `header_magic_*`.

  * **Randomized Padding:** The `transport.morphing` module appends random padding (between 40 and 280 bytes) to every initial message. Packet lengths change dynamically on every connection, invalidating static byte-matching heuristics.

  * **Pre-handshake Junk Spray:** The client dispatches 2 to 5 random junk datagrams immediately before sending the initialization packet. This causes stateful DPI state machines to misclassify the stream as unformatted entropy and ignore subsequent handshake packets.

### Threat 2: Active Scanning & Synthetic Probing

* **Mechanism:** When a middlebox detects an encrypted stream connected to an unknown external IP on port 443, it dispatches an automated scanner to probe the destination server. If the server does not present a valid TLS certificate or fails to behave like a standard web server, the IP is blacklisted.

* **Oxide Defense:**

  * **Reality Camouflage:** The `oxide-coordinator` and fallback relays implement TLS 1.3 Reality camouflage.

  * **edgeray-appProbe Diversion:** Incoming connections must present an authenticated cryptographic token (`short_id`) in the initial handshake. If an unauthenticated scanner connects, the server's Actix Web engine acts as a transparent reverse proxy, serving the authentic certificate and content of the specified `sni_target` (e.g., `www.microsoft.com`). The scanner observes a standard, compliant HTTPS endpoint.

### Threat 3: SNI Filtering & TCP Connection Resets (RST)

* **Mechanism:** Middleboxes parse plain-text Server Name Indication (SNI) records in TLS ClientHello packets to detect banned domains, immediately injecting spoofed TCP RST packets.

* **Oxide Defense:**

  * **ClientHello Fragmentation:** The `transport.fragmentation` module slices the TCP stream directly within the SNI payload (e.g., after the 3rd byte).

  * **Reassembly Defeat:** The client transmits the first fragment, sleeps for `split_delay_ms` (e.g., 4 ms), and then transmits the remainder. Most middlebox DPI engines operate without full TCP reassembly buffers to maintain line-rate performance and will fail to extract the SNI.

### Threat 4: Total UDP Blockades & Protocol Throttling

* **Mechanism:** During heightened filtering periods, network operators frequently throttle all UDP traffic to sub-usable speeds (< 50 kbps) or drop UDP packets entirely, breaking standard WireGuard and QUIC tunnels.

* **Oxide Defense:**

  * **Automatic Transport Transition:** If UDP packet delivery drops below operational thresholds for more than `udp_failure_threshold_seconds` (8 seconds), `oxide-daemon` seamlessly shifts transit to the TCP fallback engine.

  * **Actix Web WSS Tunnel:** Traffic is encapsulated inside an outbound WebSocket Secure (WSS) connection to an Actix Web reverse proxy running on port 443. The payload is indistinguishable from standard secure web application telemetry.

### Threat 5: DNS Pollution & Hijacking

* **Mechanism:** Plain-text UDP port 53 DNS queries are intercepted by intermediate routers, returning falsified IP addresses or `127.0.0.1`.

* **Oxide Defense:**

  * **Direct-IP DNS-over-HTTPS:** `oxide-dns` bypasses system resolvers and local DNS servers entirely, routing all lookups over HTTPS directly to pre-configured IP endpoints (`1.1.1.1`, `8.8.8.8`) with hardcoded IP bootstrap configurations.

## 2. Recommended Production Setup

For deployment in high-adversity regions, configure your nodes as follows:

1. **Deploy Relays on Unblocked Cloud Providers:** Run `oxide-coordinator` on subnets belonging to providers with clean IP ranges that are not pre-emptively blocked.

2. **Set SNI Target to High-Reputation Domains:** Use clean, unblocked CDN domains or cloud platform endpoints (e.g., Microsoft, Apple, AWS) for the `sni_target` setting.

3. **Enable Port Hopping:** Ensure both the client and server configuration files have identical `port_range_*` values and synchronize their system clocks via NTP to maintain alignment during scheduled hops.

4. **Keep TCP Fallback Active:** Keep `auto_switch_on_udp_failure = true` to allow uninterrupted connectivity if UDP traffic is restricted.
