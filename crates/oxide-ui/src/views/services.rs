//! Tier F-5: Embedded Network Services Management Surfaces

use dioxus::prelude::*;
use crate::components::*;
use crate::components::icons::*;
use crate::state::AppState;

#[component]
pub fn ServicesView(
    state: Signal<AppState>,
) -> Element {
    let app = state();
    let dns = &app.magic_dns;

    let mut dns_test_domain = use_signal(|| "dev-api.mesh.oxide".to_string());
    let mut dns_query_result = use_signal(|| None::<String>);

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 20px;",
            // MagicDNS Section & Query Tester
            div { class: "grid-2",
                Card {
                    title: Some("MagicDNS Mesh Resolution & Search Domains".to_string()),
                    div { style: "display: flex; flex-direction: column; gap: 14px;",
                        div { class: "metric-box",
                            div { class: "metric-label", "Node Fully Qualified MagicDNS Name" }
                            div { style: "display: flex; align-items: center; justify-content: space-between; margin-top: 4px;",
                                span { style: "font-family: var(--font-mono); font-size: 14px; font-weight: 700; color: var(--accent-cyan);", "{dns.fqdn}" }
                                Button {
                                    variant: ButtonVariant::Ghost,
                                    IconCopy { size: 14 }
                                    "Copy"
                                }
                            }
                        }

                        div {
                            div { class: "input-label", "Dynamic Aliases" }
                            div { style: "display: flex; gap: 6px; flex-wrap: wrap; margin-top: 4px;",
                                for alias in dns.aliases.iter() {
                                    Badge { variant: BadgeVariant::Muted, "{alias}" }
                                }
                            }
                        }

                        div {
                            div { class: "input-label", "Secure Upstream DNS-over-HTTPS (DoH)" }
                            div { style: "font-size: 13px; font-weight: 600; color: var(--text-primary); margin-top: 2px;", "{dns.doh_upstream}" }
                        }
                    }
                }

                Card {
                    title: Some("In-Browser MagicDNS Diagnostic Query Tester".to_string()),
                    div { style: "display: flex; flex-direction: column; gap: 12px;",
                        div { class: "input-group", style: "margin-bottom: 0;",
                            label { class: "input-label", "Lookup Mesh Hostname or Internal Domain" }
                            div { style: "display: flex; gap: 8px;",
                                input {
                                    class: "input-text input-mono",
                                    style: "flex: 1;",
                                    value: "{dns_test_domain}",
                                    oninput: move |evt| dns_test_domain.set(evt.value()),
                                }
                                Button {
                                    variant: ButtonVariant::Primary,
                                    on_click: move |_| {
                                        dns_query_result.set(Some("100.64.0.42 (Authoritative Local Lease, TTL: 60s, Latency: 0.4ms)".to_string()));
                                    },
                                    "Resolve"
                                }
                            }
                        }

                        if let Some(ref res) = dns_query_result() {
                            div { style: "padding: 12px; background: var(--bg-elevation-1); border-radius: var(--radius-sm); border: 1px solid var(--border-focus); margin-top: 4px;",
                                div { style: "font-size: 11px; color: var(--text-muted);", "A / AAAA Record Response:" }
                                div { style: "font-family: var(--font-mono); font-size: 13px; font-weight: 700; color: var(--accent-emerald); margin-top: 2px;", "{res}" }
                            }
                        }
                    }
                }
            }

            // Oxide-Drop P2P Drag-and-Drop Staging Area & Transfer Manager
            Card {
                title: Some("Oxide-Drop Multiplexed QUIC Transfer Manager".to_string()),
                div { style: "display: flex; flex-direction: column; gap: 14px;",
                    div {
                        style: "border: 2px dashed var(--border-strong); border-radius: var(--radius-md); padding: 32px 20px; text-align: center; background: var(--bg-elevation-1); transition: all 0.2s;",
                        div { style: "display: flex; flex-direction: column; align-items: center; gap: 8px;",
                            IconUpload { size: 32, color: "var(--accent-cyan)" }
                            div { style: "font-weight: 600; font-size: 14px; color: var(--text-primary);", "Drag & Drop Files Here to Stage Peer-to-Peer Transfer" }
                            div { style: "font-size: 12px; color: var(--text-muted);", "End-to-end encrypted with ChaCha20-Poly1305 and verified with BLAKE3 cryptographic hashes." }
                        }
                    }

                    for tx in app.drop_transfers.iter() {
                        div { class: "metric-box",
                            div { style: "display: flex; align-items: center; justify-content: space-between;",
                                div {
                                    div { style: "font-weight: 600; font-size: 14px; color: var(--text-primary);", "{tx.filename}" }
                                    div { style: "font-size: 12px; color: var(--text-secondary); margin-top: 2px;", "To: {tx.peer_hostname} • {(tx.file_size_bytes as f64 / 1e6):.1} MB" }
                                }
                                div { style: "text-align: right;",
                                    div { style: "font-family: var(--font-mono); font-weight: 700; font-size: 14px; color: var(--accent-emerald);", "{tx.speed_mbps:.1} MB/s" }
                                    if tx.blake3_verified {
                                        Badge { variant: BadgeVariant::Healthy, "BLAKE3 Verified" }
                                    }
                                }
                            }

                            div { style: "width: 100%; height: 6px; background: var(--bg-elevation-3); border-radius: var(--radius-full); margin-top: 10px; overflow: hidden;",
                                div { style: "width: {(tx.progress_ratio * 100.0):.0}%; height: 100%; background: linear-gradient(90deg, var(--accent-cyan), var(--accent-emerald));" }
                            }
                        }
                    }
                }
            }

            // Oxide-SSH Hardware-Accelerated Web PTY Terminal Emulator
            Card {
                title: Some("Oxide-SSH Integrated Ephemeral Terminal".to_string()),
                header_action: Some(rsx! {
                    Badge { variant: BadgeVariant::Cyan, "gateway-frankfurt.mesh.oxide (Active)" }
                }),
                div { style: "display: flex; flex-direction: column; gap: 10px;",
                    div { class: "pty-terminal",
                        div { "Connecting to gateway-frankfurt.mesh.oxide via QUIC stream #14..." }
                        div { "Authenticated with node Ed25519 identity key. Ephemeral capability token granted." }
                        div { "Linux gateway-frankfurt 6.11.0-oxide #1 SMP PREEMPT_DYNAMIC x86_64" }
                        div { "Last login: Sat Sep 26 19:12:04 2026 from 100.64.0.42" }
                        div { style: "color: #10b981; margin-top: 8px;", "root@gateway-frankfurt:~# oxide-cli status --mesh-health" }
                        div { "Overlay IPv4: 100.64.0.1 (Routing Active, 14 peers connected)" }
                        div { "DPI Evasion:  Header Scramble ACTIVE (Magic: 0x8F4E2A1B)" }
                        div { style: "color: #10b981; margin-top: 8px;", "root@gateway-frankfurt:~# _" }
                    }

                    div { style: "display: flex; gap: 8px; flex-wrap: wrap;",
                        Button { variant: ButtonVariant::Secondary, "Ctrl" }
                        Button { variant: ButtonVariant::Secondary, "Alt" }
                        Button { variant: ButtonVariant::Secondary, "Esc" }
                        Button { variant: ButtonVariant::Secondary, "Tab" }
                        Button { variant: ButtonVariant::Secondary, "Ctrl+C" }
                        Button { variant: ButtonVariant::Secondary, "Clear" }
                    }
                }
            }

            // Oxide-Serve & Oxide-Funnel Service Ingress Configurator
            Card {
                title: Some("Oxide-Serve & Oxide-Funnel Ingress Port Relays".to_string()),
                div { style: "display: flex; flex-direction: column; gap: 12px;",
                    for srv in app.ingress_services.iter() {
                        div { style: "display: flex; align-items: center; justify-content: space-between; padding: 12px 16px; background: var(--bg-elevation-1); border-radius: var(--radius-sm); border: 1px solid var(--border-subtle);",
                            div {
                                div { style: "font-weight: 600; font-size: 14px; color: var(--text-primary);", "{srv.local_bind_addr} -> {srv.magic_dns_alias}" }
                                if let Some(ref pub_url) = srv.public_url {
                                    div { style: "font-size: 12px; color: var(--accent-cyan); margin-top: 2px;", "Public URL: {pub_url}" }
                                }
                            }
                            div { style: "display: flex; align-items: center; gap: 12px;",
                                if srv.acme_tls_provisioned {
                                    Badge { variant: BadgeVariant::Healthy, "Let's Encrypt TLS Active" }
                                }
                                div { style: "font-family: var(--font-mono); font-size: 12px; color: var(--text-secondary);", "{srv.requests_handled} requests" }
                            }
                        }
                    }
                }
            }
        }
    }
}
