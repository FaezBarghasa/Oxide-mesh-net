//! Tier F-1: Node State Machine, Dynamic Peer Topology & Telemetry Engine

use dioxus::prelude::*;
use crate::components::*;
use crate::components::icons::*;
use crate::models::*;
use crate::state::AppState;

#[component]
pub fn DashboardView(
    state: Signal<AppState>,
) -> Element {
    let app = state();
    let tel = &app.telemetry;

    let format_bytes = |b: u64| -> String {
        if b >= 1_000_000_000 {
            format!("{:.2} GB", b as f64 / 1e9)
        } else if b >= 1_000_000 {
            format!("{:.2} MB", b as f64 / 1e6)
        } else if b >= 1_000 {
            format!("{:.1} KB", b as f64 / 1e3)
        } else {
            format!("{} B", b)
        }
    };

    let format_rate = |b: u64| -> String {
        format!("{}/s", format_bytes(b))
    };

    let is_connected = app.node_state == NodeState::Connected;
    let in_rate_str = format_rate(tel.ingress_bytes_sec);
    let out_rate_str = format_rate(tel.egress_bytes_sec);
    let peer_roster_title = format!("Dynamic Peer Roster ({} Nodes Active)", app.active_peers.len());

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 20px;",
            // Top Command Banner
            Card {
                div { style: "display: flex; align-items: center; justify-content: space-between; flex-wrap: wrap; gap: 16px;",
                    div { style: "display: flex; align-items: center; gap: 16px;",
                        div {
                            style: "width: 48px; height: 48px; border-radius: var(--radius-md); background: var(--bg-elevation-2); display: flex; align-items: center; justify-content: center; border: 1px solid var(--border-strong);",
                            IconShield { size: 24, color: if is_connected { "var(--accent-emerald)" } else { "var(--text-muted)" } }
                        }
                        div {
                            div { style: "display: flex; align-items: center; gap: 10px;",
                                h2 { style: "font-size: 18px; font-weight: 700;", "Oxide Mesh Overlay" }
                                match app.node_state {
                                    NodeState::Connected => rsx! { Badge { variant: BadgeVariant::Healthy, "ONLINE" } },
                                    NodeState::Connecting => rsx! { Badge { variant: BadgeVariant::Cyan, "CONNECTING" } },
                                    NodeState::Disconnecting => rsx! { Badge { variant: BadgeVariant::Warning, "DISCONNECTING" } },
                                    NodeState::Offline => rsx! { Badge { variant: BadgeVariant::Muted, "OFFLINE" } },
                                    NodeState::EmergencyObfuscation => rsx! { Badge { variant: BadgeVariant::Violet, "EMERGENCY OBFUSCATION" } },
                                    NodeState::Error => rsx! { Badge { variant: BadgeVariant::Critical, "ERROR" } },
                                }
                            }
                            div { style: "display: flex; align-items: center; gap: 14px; margin-top: 4px; font-size: 12px; color: var(--text-secondary); font-family: var(--font-mono);",
                                span { "IPv4: {tel.overlay_ipv4}" }
                                span { "•" }
                                span { "IPv6: {tel.overlay_ipv6}" }
                                span { "•" }
                                span { "MTU: {tel.current_mtu}" }
                            }
                        }
                    }

                    div { style: "display: flex; align-items: center; gap: 16px;",
                        div { style: "text-align: right;",
                            div { style: "font-size: 11px; text-transform: uppercase; color: var(--text-muted); font-weight: 600;", "NAT Discovery" }
                            div { style: "font-weight: 600; font-size: 13px; color: var(--accent-cyan);",
                                match tel.nat_type {
                                    NatType::FullCone => "Full-Cone NAT",
                                    NatType::RestrictedCone => "Restricted-Cone NAT",
                                    NatType::PortRestricted => "Port-Restricted NAT",
                                    NatType::Symmetric => "Symmetric (Hard NAT)",
                                    NatType::Unknown => "Discovering...",
                                }
                            }
                        }
                        Toggle {
                            checked: is_connected,
                            on_toggle: move |checked| {
                                let mut current = state.write();
                                current.node_state = if checked { NodeState::Connected } else { NodeState::Offline };
                            }
                        }
                    }
                }
            }

            // Real-time Metrics HUD
            div { class: "grid-4",
                div { class: "metric-box",
                    div { class: "metric-label", "Ingress Bandwidth" }
                    div { class: "metric-value", style: "color: var(--accent-cyan);", "{in_rate_str}" }
                    div { style: "font-size: 11px; color: var(--text-secondary); margin-top: 2px;", "{tel.pps_in} PPS In" }
                    Sparkline {
                        points: tel.throughput_history.clone(),
                        stroke_color: "#00f0ff",
                        fill_color: "rgba(0, 240, 255, 0.12)",
                    }
                }

                div { class: "metric-box",
                    div { class: "metric-label", "Egress Bandwidth" }
                    div { class: "metric-value", style: "color: var(--accent-emerald);", "{out_rate_str}" }
                    div { style: "font-size: 11px; color: var(--text-secondary); margin-top: 2px;", "{tel.pps_out} PPS Out" }
                    Sparkline {
                        points: tel.throughput_history.iter().map(|v| v * 0.6).collect(),
                        stroke_color: "#10b981",
                        fill_color: "rgba(16, 185, 129, 0.12)",
                    }
                }

                div { class: "metric-box",
                    div { class: "metric-label", "Path Latency (Smoothed RTT)" }
                    div { class: "metric-value", style: "color: var(--accent-amber);", "22.4 ms" }
                    div { style: "font-size: 11px; color: var(--text-secondary); margin-top: 2px;", "Jitter: ±1.2 ms" }
                    Sparkline {
                        points: tel.latency_history.clone(),
                        stroke_color: "#f59e0b",
                        fill_color: "rgba(245, 158, 11, 0.12)",
                    }
                }

                div { class: "metric-box",
                    div { class: "metric-label", "Worker Engine Threads" }
                    div { class: "metric-value", style: "color: var(--accent-violet);", "{tel.active_threads} Cores" }
                    div { style: "font-size: 11px; color: var(--text-secondary); margin-top: 2px;", "Lockless Ring Buffers Active" }
                    div { style: "margin-top: 14px; display: flex; gap: 4px;",
                        for _ in 0..tel.active_threads {
                            div { style: "flex: 1; height: 16px; border-radius: 2px; background: var(--accent-violet-glow); border: 1px solid rgba(168, 85, 247, 0.4);" }
                        }
                    }
                }
            }

            // Dynamic Peer Roster Section
            Card {
                title: peer_roster_title,
                header_action: Some(rsx! {
                    div { style: "display: flex; gap: 8px;",
                        Button {
                            variant: ButtonVariant::Ghost,
                            IconRefresh { size: 14 }
                            "Refresh"
                        }
                    }
                }),

                div { style: "display: flex; flex-direction: column; gap: 8px; margin-top: 6px;",
                    for peer in app.active_peers.iter() {
                        div {
                            class: "peer-card",
                            key: "{peer.id}",
                            onclick: {
                                let p = peer.clone();
                                move |_| {
                                    let mut current = state.write();
                                    current.selected_peer_diagnostics = Some(PeerDeepDiagnostics {
                                        peer_id: p.id.clone(),
                                        quic_version: "RFC 9000 (v1)".to_string(),
                                        congestion_algorithm: "BBRv2 (Bandwidth & RTT)".to_string(),
                                        bbr_pacing_rate_mbps: 184.2,
                                        bbr_inflight_bytes: 65536,
                                        rss_socket_pool_index: 2,
                                        public_key_ed25519: "ed25519:9f2c...4a8b".to_string(),
                                        rekey_interval_secs: 180,
                                        tpm_endorsement_verified: true,
                                        last_ping_rtt_us: Some((p.rtt_ms * 1000.0) as u32),
                                    });
                                    current.is_drawer_open = true;
                                }
                            },
                            div { class: "peer-info",
                                div { class: "peer-avatar",
                                    IconServer { size: 18 }
                                }
                                div {
                                    div { style: "display: flex; align-items: center; gap: 8px;",
                                        span { style: "font-weight: 600; font-size: 14px; color: var(--text-primary);", "{peer.hostname}" }
                                        match peer.connection_vector {
                                            PeerConnectionVector::DirectP2P => rsx! { Badge { variant: BadgeVariant::Healthy, "Direct P2P (UDP Hole-Punched)" } },
                                            PeerConnectionVector::MasqueRelayed => rsx! { Badge { variant: BadgeVariant::Warning, "MASQUE Relayed (HTTP/3)" } },
                                            PeerConnectionVector::FallbackWss => rsx! { Badge { variant: BadgeVariant::Violet, "WSS Fallback (TCP:443)" } },
                                        }
                                    }
                                    div { style: "font-size: 12px; color: var(--text-secondary); font-family: var(--font-mono); margin-top: 2px;",
                                        span { "{peer.overlay_ip}" }
                                        span { " • " }
                                        span { "{peer.remote_endpoint}" }
                                        span { " • " }
                                        span { "{peer.os}" }
                                    }
                                }
                            }

                            div { style: "display: flex; align-items: center; gap: 20px;",
                                div { style: "text-align: right;",
                                    div { style: "font-size: 11px; color: var(--text-muted); font-weight: 600;", "RTT" }
                                    div { style: "font-family: var(--font-mono); font-weight: 700; color: var(--text-primary); font-size: 13px;", "{peer.rtt_ms:.1} ms" }
                                }
                                div { style: "text-align: right;",
                                    div { style: "font-size: 11px; color: var(--text-muted); font-weight: 600;", "Handshake" }
                                    div { style: "font-size: 12px; color: var(--accent-emerald); font-weight: 500;", "{peer.last_handshake_secs}s ago" }
                                }
                                IconChevronRight { size: 16, color: "var(--text-muted)" }
                            }
                        }
                    }
                }
            }

            // Slide-out Diagnostic Drawer for Selected Peer
            Drawer {
                is_open: app.is_drawer_open,
                title: "Peer Deep Diagnostic Inspector".to_string(),
                on_close: move |_| {
                    let mut current = state.write();
                    current.is_drawer_open = false;
                },
                if let Some(diag) = &app.selected_peer_diagnostics {
                    div { style: "display: flex; flex-direction: column; gap: 14px;",
                        div { class: "metric-box",
                            div { class: "metric-label", "Peer Identifier" }
                            div { style: "font-family: var(--font-mono); font-size: 13px; color: var(--accent-cyan); margin-top: 4px;", "{diag.peer_id}" }
                        }

                        div { class: "grid-2",
                            div { class: "metric-box",
                                div { class: "metric-label", "QUIC Transport Version" }
                                div { style: "font-size: 13px; font-weight: 600; margin-top: 4px;", "{diag.quic_version}" }
                            }
                            div { class: "metric-box",
                                div { class: "metric-label", "Congestion Algorithm" }
                                div { style: "font-size: 13px; font-weight: 600; margin-top: 4px;", "{diag.congestion_algorithm}" }
                            }
                        }

                        div { class: "grid-2",
                            div { class: "metric-box",
                                div { class: "metric-label", "BBR Pacing Rate" }
                                div { style: "font-size: 14px; font-weight: 700; color: var(--accent-emerald); font-family: var(--font-mono); margin-top: 4px;", "{diag.bbr_pacing_rate_mbps:.1} Mbps" }
                            }
                            div { class: "metric-box",
                                div { class: "metric-label", "Inflight Bytes" }
                                div { style: "font-size: 14px; font-weight: 700; color: var(--text-primary); font-family: var(--font-mono); margin-top: 4px;", "{diag.bbr_inflight_bytes} B" }
                            }
                        }

                        div { class: "metric-box",
                            div { class: "metric-label", "Cryptographic Public Key" }
                            div { style: "font-family: var(--font-mono); font-size: 11px; color: var(--text-secondary); word-break: break-all; margin-top: 4px;", "{diag.public_key_ed25519}" }
                        }

                        div { style: "display: flex; align-items: center; justify-content: space-between; padding: 12px; background: var(--bg-elevation-1); border-radius: var(--radius-sm); border: 1px solid var(--border-subtle);",
                            span { style: "font-size: 13px; font-weight: 500;", "TPM 2.0 Hardware Endorsement" }
                            if diag.tpm_endorsement_verified {
                                Badge { variant: BadgeVariant::Healthy, "VERIFIED" }
                            } else {
                                Badge { variant: BadgeVariant::Warning, "UNVERIFIED" }
                            }
                        }

                        Button {
                            variant: ButtonVariant::Primary,
                            on_click: move |_| {
                                // Diagnostic Ping Trigger
                            },
                            IconActivity { size: 16 }
                            "Trigger Instant QUIC Path Probe"
                        }
                    }
                }
            }
        }
    }
}
