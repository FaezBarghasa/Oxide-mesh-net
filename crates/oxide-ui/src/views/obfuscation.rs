//! Tier F-3: DPI Circumvention, Anti-Censorship & Obfuscation Panel

use dioxus::prelude::*;
use crate::components::*;
use crate::components::icons::*;
use crate::state::AppState;

#[component]
pub fn ObfuscationView(
    state: Signal<AppState>,
) -> Element {
    let app = state();
    let dpi = &app.dpi_config;
    let reality = &app.reality_config;
    let frag = &app.frag_config;
    let hop = &app.port_hop_state;

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 20px;",
            // High Adversity Warning Banner
            div { style: "display: flex; align-items: center; justify-content: space-between; padding: 14px 20px; background: linear-gradient(135deg, rgba(168, 85, 247, 0.15), rgba(0, 240, 255, 0.15)); border: 1px solid var(--accent-violet); border-radius: var(--radius-md);",
                div { style: "display: flex; align-items: center; gap: 14px;",
                    IconLock { size: 24, color: "var(--accent-violet)" }
                    div {
                        div { style: "font-weight: 700; font-size: 15px; color: var(--text-primary);", "State-Level DPI Evasion & Transport Stealth Active" }
                        div { style: "font-size: 12px; color: var(--text-secondary); margin-top: 2px;", "Packet transformations invalidate heuristic middlebox fingerprinting and shallow SNI inspectors." }
                    }
                }
                Badge { variant: BadgeVariant::Violet, "STEALTH CAMOUFLAGE" }
            }

            // Protocol Morphing & Signature Scrambler
            div { class: "grid-2",
                Card {
                    title: Some("Header Magic Scrambling & Packet Padding".to_string()),
                    div { style: "display: flex; flex-direction: column; gap: 16px;",
                        div { style: "display: flex; align-items: center; justify-content: space-between;",
                            div {
                                div { style: "font-weight: 600; font-size: 13px;", "Header Magic Scrambling" }
                                div { style: "font-size: 11px; color: var(--text-muted);", "Strips standard WireGuard / QUIC fixed byte signatures" }
                            }
                            Toggle {
                                checked: dpi.magic_header_scramble_enabled,
                                on_toggle: move |v| {
                                    let mut current = state.write();
                                    current.dpi_config.magic_header_scramble_enabled = v;
                                }
                            }
                        }

                        div { class: "input-group",
                            label { class: "input-label", "Current Cryptographic Magic Discriminator (4-Byte Hex)" }
                            input {
                                class: "input-text input-mono",
                                value: "{dpi.custom_magic_hex}",
                                readonly: true,
                            }
                        }

                        div {
                            div { class: "input-label", "Variable Packet Padding Boundary (Min / Max Bytes)" }
                            div { style: "display: flex; flex-direction: column; gap: 8px; margin-top: 6px;",
                                Slider {
                                    value: dpi.padding_min_bytes as f64,
                                    min: 0.0,
                                    max: 128.0,
                                    step: Some(8.0),
                                    unit: Some("B Min".to_string()),
                                    on_change: move |v| {
                                        let mut current = state.write();
                                        current.dpi_config.padding_min_bytes = v as u16;
                                    }
                                }
                                Slider {
                                    value: dpi.padding_max_bytes as f64,
                                    min: 128.0,
                                    max: 512.0,
                                    step: Some(16.0),
                                    unit: Some("B Max".to_string()),
                                    on_change: move |v| {
                                        let mut current = state.write();
                                        current.dpi_config.padding_max_bytes = v as u16;
                                    }
                                }
                            }
                        }

                        div { style: "display: flex; align-items: center; justify-content: space-between; padding-top: 8px; border-top: 1px solid var(--border-subtle);",
                            div {
                                div { style: "font-weight: 600; font-size: 13px;", "Pre-Handshake Junk Datagram Injection" }
                                div { style: "font-size: 11px; color: var(--text-muted);", "Dispatches {dpi.junk_burst_count} random entropy frames prior to handshake" }
                            }
                            Toggle {
                                checked: dpi.pre_handshake_junk_enabled,
                                on_toggle: move |v| {
                                    let mut current = state.write();
                                    current.dpi_config.pre_handshake_junk_enabled = v;
                                }
                            }
                        }
                    }
                }

                // Reality TLS 1.3 Camouflage & Active Probing Defense
                Card {
                    title: Some("Reality TLS 1.3 Camouflage & Active Probing Defense".to_string()),
                    div { style: "display: flex; flex-direction: column; gap: 14px;",
                        div { class: "input-group", style: "margin-bottom: 0;",
                            label { class: "input-label", "High-Reputation SNI Target Endpoint" }
                            input {
                                class: "input-text input-mono",
                                value: "{reality.sni_target}",
                            }
                        }

                        div { style: "display: flex; align-items: center; justify-content: space-between; padding: 10px 14px; background: var(--bg-elevation-1); border-radius: var(--radius-sm); border: 1px solid var(--border-subtle);",
                            div {
                                div { style: "font-size: 12px; font-weight: 600;", "SNI Target Reachability & TLS 1.3 ALPN" }
                                div { style: "font-size: 11px; color: var(--text-muted); margin-top: 2px;", "Verified HTTP/2 & HTTP/3 support" }
                            }
                            if reality.remote_sni_reachable {
                                Badge { variant: BadgeVariant::Healthy, "ONLINE & REACHABLE" }
                            } else {
                                Badge { variant: BadgeVariant::Critical, "UNREACHABLE" }
                            }
                        }

                        div { class: "grid-2",
                            div { class: "metric-box",
                                div { class: "metric-label", "Authentication Short ID" }
                                div { style: "font-family: var(--font-mono); font-size: 12px; color: var(--accent-cyan); margin-top: 4px;", "{reality.short_id_hex}" }
                            }
                            div { class: "metric-box",
                                div { class: "metric-label", "Diverted Active Probes" }
                                div { style: "font-family: var(--font-mono); font-size: 16px; font-weight: 700; color: var(--accent-emerald); margin-top: 2px;", "{reality.diverted_probes_count} Probes" }
                            }
                        }

                        div { class: "input-group", style: "margin-bottom: 0;",
                            label { class: "input-label", "Reality Server Public Key (x25519)" }
                            input {
                                class: "input-text input-mono",
                                style: "font-size: 11px;",
                                value: "{reality.server_public_key_hex}",
                                readonly: true,
                            }
                        }
                    }
                }
            }

            // ClientHello TCP Fragmentation & Dynamic Port Hopping Timeline
            div { class: "grid-2",
                Card {
                    title: Some("ClientHello TCP SNI Split & Delay Tuner".to_string()),
                    div { style: "display: flex; flex-direction: column; gap: 14px;",
                        div { style: "font-size: 12px; color: var(--text-secondary);",
                            "Cleaves initial TCP ClientHello frames across segment boundaries with microsecond delays to defeat shallow SNI matching."
                        }

                        div {
                            div { class: "input-label", "SNI Split Byte Boundary Offset" }
                            div { style: "margin-top: 6px;",
                                Slider {
                                    value: frag.split_byte_offset as f64,
                                    min: 1.0,
                                    max: 64.0,
                                    step: Some(1.0),
                                    unit: Some("Bytes".to_string()),
                                    on_change: move |v| {
                                        let mut current = state.write();
                                        current.frag_config.split_byte_offset = v as u16;
                                    }
                                }
                            }
                        }

                        div {
                            div { class: "input-label", "Transmission Delay Between Split Segments" }
                            div { style: "margin-top: 6px;",
                                Slider {
                                    value: frag.split_delay_ms as f64,
                                    min: 1.0,
                                    max: 20.0,
                                    step: Some(1.0),
                                    unit: Some("ms".to_string()),
                                    on_change: move |v| {
                                        let mut current = state.write();
                                        current.frag_config.split_delay_ms = v as u16;
                                    }
                                }
                            }
                        }

                        div { style: "display: flex; align-items: center; justify-content: space-between; padding: 10px 14px; background: var(--bg-elevation-1); border-radius: var(--radius-sm); border: 1px solid var(--border-subtle);",
                            span { style: "font-size: 13px; font-weight: 500;", "Active TCP Frame Splitting" }
                            if frag.is_actively_splitting {
                                Badge { variant: BadgeVariant::Cyan, "ACTIVE" }
                            } else {
                                Badge { variant: BadgeVariant::Muted, "STANDBY" }
                            }
                        }
                    }
                }

                Card {
                    title: Some("Dynamic Port Hopping & Emergency WSS Failover".to_string()),
                    div { style: "display: flex; flex-direction: column; gap: 14px;",
                        div { style: "display: flex; align-items: center; justify-content: space-between;",
                            div {
                                div { style: "font-size: 11px; color: var(--text-muted); font-weight: 600;", "ACTIVE UDP PORT" }
                                div { style: "font-family: var(--font-mono); font-size: 22px; font-weight: 700; color: var(--accent-cyan);", "{hop.current_port}" }
                            }
                            div { style: "text-align: right;",
                                div { style: "font-size: 11px; color: var(--text-muted); font-weight: 600;", "NEXT HOP CANDIDATE" }
                                div { style: "font-family: var(--font-mono); font-size: 18px; font-weight: 600; color: var(--accent-violet);", "{hop.next_port_candidate}" }
                                div { style: "font-size: 11px; color: var(--text-secondary);", "in {hop.seconds_to_next_hop} seconds" }
                            }
                        }

                        div { style: "padding: 12px; background: var(--bg-elevation-1); border-radius: var(--radius-sm); border: 1px solid var(--border-subtle); display: flex; flex-direction: column; gap: 8px;",
                            div { style: "font-size: 12px; font-weight: 600;", "Emergency Transport Mode (WSS Tunnel on Port 443)" }
                            div { style: "font-size: 11px; color: var(--text-secondary);",
                                "If state firewalls drop or throttle all UDP traffic, traffic seamlessly shifts to Actix Web WebSocket Secure (WSS/443)."
                            }
                            div { style: "display: flex; align-items: center; justify-content: space-between; margin-top: 4px;",
                                span { style: "font-size: 12px; font-weight: 500;", "Enforce HTTPS/WSS Only" }
                                Toggle {
                                    checked: hop.enforce_wss_only,
                                    on_toggle: move |v| {
                                        let mut current = state.write();
                                        current.port_hop_state.enforce_wss_only = v;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
