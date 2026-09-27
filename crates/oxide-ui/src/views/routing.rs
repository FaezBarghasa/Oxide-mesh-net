//! Tier F-2: Routing Fabric, Exit Node Selection & Subnet Governance

use crate::components::icons::*;
use crate::components::*;
use crate::models::*;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn RoutingView(state: Signal<AppState>) -> Element {
    let app = state();
    let mut cidr_input = use_signal(|| "192.168.10.0/24".to_string());
    let mut test_query = use_signal(|| "104.244.42.1".to_string());

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 20px;",
            // Exit Node Selector Card
            Card {
                title: Some("Exit Node Selector & Global Gateway Controller".to_string()),
                div { style: "display: flex; flex-direction: column; gap: 14px;",
                    div { style: "font-size: 13px; color: var(--text-secondary);",
                        "Route all outbound internet traffic (0.0.0.0/0, ::/0) through a verified mesh peer."
                    }

                    for exit in app.exit_nodes.iter() {
                        div {
                            class: "peer-card",
                            key: "{exit.id}",
                            div { class: "peer-info",
                                div { class: "peer-avatar",
                                    IconGlobe { size: 18 }
                                }
                                div {
                                    div { style: "display: flex; align-items: center; gap: 8px;",
                                        span { style: "font-weight: 600; font-size: 14px;", "{exit.hostname}" }
                                        if exit.is_active {
                                            Badge { variant: BadgeVariant::Healthy, "ACTIVE EXIT GATEWAY" }
                                        }
                                        if exit.dns_leak_protected {
                                            Badge { variant: BadgeVariant::Cyan, "DNS LEAK PROTECTED" }
                                        }
                                    }
                                    div { style: "font-size: 12px; color: var(--text-secondary); margin-top: 2px;",
                                        span { "{exit.location_city}, {exit.location_country}" }
                                        if let Some(ref pub_ip) = exit.verified_public_ip {
                                            span { " • Public IP: {pub_ip}" }
                                        }
                                    }
                                }
                            }

                            div { style: "display: flex; align-items: center; gap: 16px;",
                                div { style: "text-align: right;",
                                    div { style: "font-size: 11px; color: var(--text-muted); font-weight: 600;", "LATENCY" }
                                    div { style: "font-family: var(--font-mono); font-weight: 700; color: var(--text-primary);", "{exit.latency_ms:.1} ms" }
                                }
                                Button {
                                    variant: if exit.is_active { ButtonVariant::Primary } else { ButtonVariant::Secondary },
                                    on_click: {
                                        let id = exit.id.clone();
                                        move |_| {
                                            let mut current = state.write();
                                            for e in current.exit_nodes.iter_mut() {
                                                e.is_active = (e.id == id) && !e.is_active;
                                            }
                                        }
                                    },
                                    if exit.is_active { "Active" } else { "Use as Exit Node" }
                                }
                            }
                        }
                    }

                    div { style: "display: flex; align-items: center; justify-content: space-between; padding: 12px 16px; background: var(--bg-elevation-1); border-radius: var(--radius-sm); border: 1px solid var(--border-subtle); margin-top: 4px;",
                        div {
                            div { style: "font-weight: 600; font-size: 13px;", "Allow Local Network Access (Split-Routing)" }
                            div { style: "font-size: 12px; color: var(--text-secondary); margin-top: 2px;", "Preserve direct access to local printers, NAS, and LAN devices while exit node is engaged." }
                        }
                        Toggle {
                            checked: true,
                            on_toggle: |_| {}
                        }
                    }
                }
            }

            // Subnet Router Advertisement & Bridge Manager
            Card {
                title: Some("Subnet Router Advertisement & Local Network Bridge".to_string()),
                div { style: "display: flex; flex-direction: column; gap: 14px;",
                    div { style: "display: flex; gap: 10px; align-items: flex-end;",
                        div { class: "input-group", style: "flex: 1; margin-bottom: 0;",
                            label { class: "input-label", "Local CIDR Range to Advertise into Mesh" }
                            input {
                                class: "input-text input-mono",
                                value: "{cidr_input}",
                                oninput: move |evt| cidr_input.set(evt.value())
                            }
                        }
                        Button {
                            variant: ButtonVariant::Primary,
                            on_click: move |_| {
                                let mut current = state.write();
                                current.advertised_subnets.push(AdvertisedSubnet {
                                    cidr: cidr_input(),
                                    interface_name: "eth0".to_string(),
                                    is_active: true,
                                    has_conflict: false,
                                    active_vrrp_leader: Some("local-node".to_string()),
                                });
                            },
                            IconUpload { size: 14 }
                            "Advertise Subnet"
                        }
                    }

                    div { style: "margin-top: 6px;",
                        table { class: "data-table",
                            thead {
                                tr {
                                    th { "Advertised Subnet CIDR" }
                                    th { "Physical Interface" }
                                    th { "VRRP Failover Status" }
                                    th { "Route Conflict" }
                                    th { "Action" }
                                }
                            }
                            tbody {
                                for (idx, sub) in app.advertised_subnets.iter().enumerate() {
                                    tr { key: "{sub.cidr}",
                                        td { style: "font-family: var(--font-mono); font-weight: 600; color: var(--text-primary);", "{sub.cidr}" }
                                        td { "{sub.interface_name}" }
                                        td {
                                            if let Some(ref leader) = sub.active_vrrp_leader {
                                                Badge { variant: BadgeVariant::Healthy, "VRRP Leader ({leader})" }
                                            } else {
                                                Badge { variant: BadgeVariant::Warning, "Standby" }
                                            }
                                        }
                                        td {
                                            if sub.has_conflict {
                                                Badge { variant: BadgeVariant::Critical, "Overlap Detected" }
                                            } else {
                                                Badge { variant: BadgeVariant::Muted, "Clean" }
                                            }
                                        }
                                        td {
                                            Button {
                                                variant: ButtonVariant::Danger,
                                                on_click: move |_| {
                                                    let mut current = state.write();
                                                    if idx < current.advertised_subnets.len() {
                                                        current.advertised_subnets.remove(idx);
                                                    }
                                                },
                                                "Withdraw"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Split-Horizon Routing & Dynamic Policy Decision Simulator
            div { class: "grid-2",
                Card {
                    title: Some("Split-Horizon & Per-App Bypass Rules".to_string()),
                    div { style: "display: flex; flex-direction: column; gap: 10px;",
                        for rule in app.split_rules.iter() {
                            div { style: "display: flex; align-items: center; justify-content: space-between; padding: 10px 14px; background: var(--bg-elevation-1); border-radius: var(--radius-sm); border: 1px solid var(--border-subtle);",
                                div {
                                    div { style: "font-family: var(--font-mono); font-size: 13px; font-weight: 600; color: var(--accent-cyan);", "{rule.pattern}" }
                                    if let Some(ref proc_name) = rule.target_app_process {
                                        div { style: "font-size: 11px; color: var(--text-muted); margin-top: 2px;", "Scoped to process: {proc_name}" }
                                    }
                                }
                                Badge { variant: BadgeVariant::Muted, "Direct Internet Bypass" }
                            }
                        }
                    }
                }

                Card {
                    title: Some("Dynamic Route Decision Tester".to_string()),
                    div { style: "display: flex; flex-direction: column; gap: 12px;",
                        div { class: "input-group", style: "margin-bottom: 0;",
                            label { class: "input-label", "Target IP or Domain Name" }
                            div { style: "display: flex; gap: 8px;",
                                input {
                                    class: "input-text input-mono",
                                    style: "flex: 1;",
                                    value: "{test_query}",
                                    oninput: move |evt| test_query.set(evt.value())
                                }
                                Button {
                                    variant: ButtonVariant::Primary,
                                    on_click: move |_| {
                                        let mut current = state.write();
                                        current.route_test_result = Some(RouteDecisionTestResult {
                                            query: test_query(),
                                            decision: "Encapsulate via Mesh TUN (wg0)".to_string(),
                                            matching_rule: Some("Default Mesh Overlay Route".to_string()),
                                            egress_interface: "oxide-tun0 (100.64.0.42)".to_string(),
                                        });
                                    },
                                    "Simulate"
                                }
                            }
                        }

                        if let Some(ref res) = app.route_test_result {
                            div { style: "padding: 12px; background: var(--bg-elevation-1); border-radius: var(--radius-sm); border: 1px solid var(--border-focus); display: flex; flex-direction: column; gap: 6px;",
                                div { style: "font-size: 12px; color: var(--text-secondary);", "Query: {res.query}" }
                                div { style: "font-weight: 700; color: var(--accent-emerald); font-size: 14px;", "{res.decision}" }
                                div { style: "font-size: 11px; color: var(--text-muted);", "Egress Path: {res.egress_interface}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
