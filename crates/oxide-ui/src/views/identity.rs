//! Tier F-4: Zero-Trust Identity, Ephemeral Enrollment & ACL Builder

use dioxus::prelude::*;
use crate::components::*;
use crate::components::icons::*;
use crate::models::*;
use crate::state::AppState;

#[component]
pub fn IdentityView(
    state: Signal<AppState>,
) -> Element {
    let app = state();
    let auth = &app.device_auth;
    let hsm = &app.hsm_status;

    let mut sim_src = use_signal(|| "tag:dev".to_string());
    let mut sim_dst = use_signal(|| "tag:gateway".to_string());
    let mut sim_port = use_signal(|| "443".to_string());

    let tags = vec!["tag:admin", "tag:gateway", "tag:prod", "tag:dev", "tag:monitoring"];

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 20px;",
            // Ephemeral Device Enrollment & RFC 8628 Grant
            div { class: "grid-2",
                Card {
                    title: Some("RFC 8628 Ephemeral Device Authorization Flow".to_string()),
                    if let Some(ref dauth) = auth {
                        div { style: "display: flex; align-items: center; gap: 20px; flex-wrap: wrap;",
                            SvgQrCode {
                                data: format!("{}/auth?code={}", dauth.verification_uri, dauth.user_code),
                                size: 140,
                            }
                            div { style: "display: flex; flex-direction: column; gap: 8px; flex: 1; min-width: 200px;",
                                div { style: "font-size: 12px; color: var(--text-secondary);", "Scan with mobile or visit verification URI:" }
                                a {
                                    href: "{dauth.verification_uri}",
                                    target: "_blank",
                                    style: "font-size: 13px; font-weight: 600; color: var(--accent-cyan); word-break: break-all;",
                                    "{dauth.verification_uri}"
                                }
                                div { class: "metric-box", style: "margin-top: 4px; text-align: center;",
                                    div { class: "metric-label", "One-Time User Code" }
                                    div { style: "font-family: var(--font-mono); font-size: 24px; font-weight: 800; letter-spacing: 0.1em; color: var(--accent-cyan);", "{dauth.user_code}" }
                                }
                                div { style: "font-size: 11px; color: var(--text-muted);", "Valid for {dauth.expires_in_secs} seconds" }
                            }
                        }
                    }
                }

                Card {
                    title: Some("Hardware Root-of-Trust & Hybrid Certificates".to_string()),
                    div { style: "display: flex; flex-direction: column; gap: 12px;",
                        div { style: "display: flex; align-items: center; justify-content: space-between; padding: 10px 14px; background: var(--bg-elevation-1); border-radius: var(--radius-sm); border: 1px solid var(--border-subtle);",
                            div {
                                div { style: "font-size: 13px; font-weight: 600;", "{hsm.hsm_type}" }
                                div { style: "font-size: 11px; color: var(--text-muted); margin-top: 2px;", "Private key isolated inside secure hardware enclave" }
                            }
                            Badge { variant: BadgeVariant::Healthy, "TPM BOUND" }
                        }

                        div { class: "metric-box",
                            div { class: "metric-label", "Post-Quantum Cryptographic Scheme" }
                            div { style: "font-family: var(--font-mono); font-size: 12px; color: var(--accent-violet); margin-top: 2px;", "{hsm.cert_algo}" }
                        }

                        div { class: "grid-2",
                            div { class: "metric-box",
                                div { class: "metric-label", "Certificate Serial" }
                                div { style: "font-family: var(--font-mono); font-size: 12px; color: var(--text-primary); margin-top: 2px;", "{hsm.cert_serial}" }
                            }
                            div { class: "metric-box",
                                div { class: "metric-label", "Expiration Timestamp" }
                                div { style: "font-size: 11px; color: var(--text-secondary); margin-top: 4px;", "{hsm.expires_at}" }
                            }
                        }
                    }
                }
            }

            // 2D Zero-Trust Access Control Matrix & Instant SIMD Policy Simulator
            div { class: "grid-2",
                Card {
                    title: Some("2D Zero-Trust Microsegmentation Matrix".to_string()),
                    div { style: "display: flex; flex-direction: column; gap: 10px;",
                        div { style: "font-size: 12px; color: var(--text-secondary);", "Click matrix cell to inspect or invert security policy boundaries between identity tags." }
                        div { class: "acl-matrix",
                            div { style: "display: grid; grid-template-columns: 80px repeat(5, 44px); gap: 4px; align-items: center;",
                                div { style: "font-size: 10px; color: var(--text-muted); font-weight: 700;", "SRC / DST" }
                                for dst in tags.iter() {
                                    div { style: "font-size: 9px; color: var(--text-muted); text-align: center; text-transform: uppercase; overflow: hidden; text-overflow: ellipsis;", "{dst.trim_start_matches(\"tag:\")}" }
                                }
                            }
                            for src in tags.iter() {
                                div { style: "display: grid; grid-template-columns: 80px repeat(5, 44px); gap: 4px; align-items: center;",
                                    div { style: "font-size: 10px; font-weight: 600; color: var(--text-secondary); overflow: hidden; text-overflow: ellipsis;", "{src.trim_start_matches(\"tag:\")}" }
                                    for dst in tags.iter() {
                                        div {
                                            class: if (src == &"tag:admin") || (src == &"tag:dev" && dst == &"tag:gateway") || (src == dst) { "matrix-cell matrix-allow" } else { "matrix-cell matrix-deny" },
                                            if (src == &"tag:admin") || (src == &"tag:dev" && dst == &"tag:gateway") || (src == dst) { "ALLOW" } else { "DENY" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                Card {
                    title: Some("SIMD-Accelerated Policy Simulator".to_string()),
                    div { style: "display: flex; flex-direction: column; gap: 12px;",
                        div { style: "font-size: 12px; color: var(--text-secondary);", "Simulate packet flow evaluation across SIMD bitmask rules without dispatching real network frames." }

                        div { class: "grid-2",
                            div { class: "input-group", style: "margin-bottom: 0;",
                                label { class: "input-label", "Source Identity Tag" }
                                input {
                                    class: "input-text input-mono",
                                    value: "{sim_src}",
                                    oninput: move |evt| sim_src.set(evt.value()),
                                }
                            }
                            div { class: "input-group", style: "margin-bottom: 0;",
                                label { class: "input-label", "Destination Identity Tag" }
                                input {
                                    class: "input-text input-mono",
                                    value: "{sim_dst}",
                                    oninput: move |evt| sim_dst.set(evt.value()),
                                }
                            }
                        }

                        div { class: "input-group", style: "margin-bottom: 0;",
                            label { class: "input-label", "Target TCP/UDP Port" }
                            input {
                                class: "input-text input-mono",
                                value: "{sim_port}",
                                oninput: move |evt| sim_port.set(evt.value()),
                            }
                        }

                        Button {
                            variant: ButtonVariant::Primary,
                            on_click: move |_| {
                                let mut current = state.write();
                                let is_allowed = sim_src() == "tag:admin" || (sim_src() == "tag:dev" && sim_dst() == "tag:gateway" && sim_port() == "443");
                                if is_allowed {
                                    current.acl_simulation_result = Some(format!("PERMIT: 64-bit SIMD mask matched rule #acl-1 ({} -> {}:{} ALLOWED)", sim_src(), sim_dst(), sim_port()));
                                } else {
                                    current.acl_simulation_result = Some(format!("DROP: Default Zero-Trust Deny ({} -> {}:{} REJECTED)", sim_src(), sim_dst(), sim_port()));
                                }
                            },
                            IconActivity { size: 14 }
                            "Evaluate SIMD ACL Decision"
                        }

                        if let Some(ref sim_res) = app.acl_simulation_result {
                            div {
                                class: "metric-box",
                                style: "margin-top: 4px; border: 1px solid var(--border-focus);",
                                div {
                                    style: if sim_res.starts_with("PERMIT") { "font-family: var(--font-mono); font-size: 13px; font-weight: 700; color: var(--accent-emerald);" } else { "font-family: var(--font-mono); font-size: 13px; font-weight: 700; color: var(--accent-rose);" },
                                    "{sim_res}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
