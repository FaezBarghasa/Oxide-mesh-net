//! Tier F-7: Cryptographic Auditing, Diagnostics & Forensic Tools

use crate::components::icons::*;
use crate::components::*;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn AuditView(state: Signal<AppState>) -> Element {
    let app = state();
    let mut verified_row = use_signal(|| None::<u64>);

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 20px;",
            // Merkle-Tree Ledger & Cryptographic Audit Explorer
            Card {
                title: Some("Merkle-Tree Cryptographic Audit Ledger".to_string()),
                div { style: "display: flex; flex-direction: column; gap: 12px;",
                    div { style: "font-size: 12px; color: var(--text-secondary);",
                        "Immutable, cryptographically verifiable event ledger signed with node Ed25519 identities and anchored to the coordinator Merkle root."
                    }

                    table { class: "data-table",
                        thead {
                            tr {
                                th { "Sequence #" }
                                th { "Timestamp" }
                                th { "Event Action" }
                                th { "Initiator Identity" }
                                th { "Merkle Leaf Hash" }
                                th { "Inclusion Proof" }
                            }
                        }
                        tbody {
                            for entry in app.merkle_logs.iter() {
                                tr { key: "{entry.sequence_num}",
                                    td { style: "font-family: var(--font-mono); font-weight: 700; color: var(--accent-cyan);", "#{entry.sequence_num}" }
                                    td { style: "font-size: 12px;", "{entry.timestamp}" }
                                    td {
                                        Badge { variant: BadgeVariant::Muted, "{entry.action_type}" }
                                    }
                                    td { style: "font-family: var(--font-mono); font-size: 11px;", "{entry.initiator_node_id}" }
                                    td { style: "font-family: var(--font-mono); font-size: 11px; color: var(--text-muted);", "{entry.merkle_leaf_hash}" }
                                    td {
                                        if verified_row() == Some(entry.sequence_num) {
                                            Badge { variant: BadgeVariant::Healthy, "ROOT HASH VERIFIED" }
                                        } else {
                                            Button {
                                                variant: ButtonVariant::Ghost,
                                                on_click: {
                                                    let seq = entry.sequence_num;
                                                    move |_| {
                                                        verified_row.set(Some(seq));
                                                    }
                                                },
                                                IconCheck { size: 14 }
                                                "Verify Proof"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Real-Time Packet Stream & Filter Debugger
            Card {
                title: Some("Real-Time Virtual Interface Packet Stream Debugger".to_string()),
                header_action: Some(rsx! {
                    Badge { variant: BadgeVariant::Cyan, "Capturing (oxide-tun0)" }
                }),
                div { style: "display: flex; flex-direction: column; gap: 12px;",
                    div { class: "input-group", style: "margin-bottom: 0;",
                        label { class: "input-label", "Packet Filter Expression (IP, Port, Protocol, or 'is:dropped')" }
                        input {
                            class: "input-text input-mono",
                            placeholder: "e.g. proto:tcp or dst:100.64.0.2 or is:dropped",
                        }
                    }

                    table { class: "data-table",
                        thead {
                            tr {
                                th { "Timestamp (μs)" }
                                th { "Protocol" }
                                th { "Source IP:Port" }
                                th { "Destination IP:Port" }
                                th { "Length" }
                                th { "Verdict / Drop Reason" }
                            }
                        }
                        tbody {
                            for pkt in app.packet_stream.iter() {
                                tr { key: "{pkt.timestamp_us}",
                                    td { style: "font-family: var(--font-mono); font-size: 11px; color: var(--text-muted);", "{pkt.timestamp_us}" }
                                    td {
                                        Badge { variant: BadgeVariant::Muted, "{pkt.protocol}" }
                                    }
                                    td { style: "font-family: var(--font-mono); font-size: 12px;", "{pkt.src_ip}:{pkt.src_port}" }
                                    td { style: "font-family: var(--font-mono); font-size: 12px;", "{pkt.dst_ip}:{pkt.dst_port}" }
                                    td { style: "font-family: var(--font-mono); font-size: 12px;", "{pkt.payload_bytes} B" }
                                    td {
                                        if pkt.is_dropped {
                                            if let Some(ref reason) = pkt.drop_reason {
                                                Badge { variant: BadgeVariant::Critical, "{reason}" }
                                            } else {
                                                Badge { variant: BadgeVariant::Critical, "DROPPED" }
                                            }
                                        } else {
                                            Badge { variant: BadgeVariant::Healthy, "FORWARDED" }
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
}
