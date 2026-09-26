//! Main Dioxus application shell and route coordinator for oxide-ui

use dioxus::prelude::*;
use crate::components::*;
use crate::components::icons::*;
use crate::models::NodeState;
use crate::state::{ActiveView, AppState};
use crate::theme::EMBEDDED_CSS;
use crate::views::*;

#[component]
pub fn App() -> Element {
    let mut state = use_signal(AppState::default);
    let app = state();

    rsx! {
        // Embedded zero-bloat hardware accelerated design token CSS
        style { "{EMBEDDED_CSS}" }

        div { class: "app-shell",
            // App Header
            header { class: "app-header",
                div { class: "brand-section",
                    div { class: "brand-badge",
                        IconShield { size: 16, color: "#070709" }
                    }
                    span { "OXIDE MESH NET" }
                    Badge { variant: BadgeVariant::Cyan, "v0.1.0-pure-rust" }
                }

                div { class: "header-telemetry",
                    div { style: "display: flex; align-items: center; gap: 8px;",
                        div {
                            style: if app.node_state == NodeState::Connected {
                                "width: 8px; height: 8px; border-radius: 50%; background: var(--accent-emerald); box-shadow: 0 0 8px var(--accent-emerald);"
                            } else {
                                "width: 8px; height: 8px; border-radius: 50%; background: var(--accent-rose); box-shadow: none;"
                            }
                        }
                        span { style: "font-size: 12px; font-weight: 600; color: var(--text-secondary);",
                            if app.node_state == NodeState::Connected { "DAEMON SYNCED" } else { "OFFLINE" }
                        }
                    }

                    span { style: "font-family: var(--font-mono); font-size: 12px; color: var(--accent-cyan);",
                        "{app.telemetry.overlay_ipv4}"
                    }
                }
            }

            // Desktop Sidebar Navigation
            aside { class: "app-sidebar",
                div { class: "nav-group",
                    div { class: "nav-label", "Mesh Network" }

                    div {
                        class: if app.current_view == ActiveView::Dashboard { "nav-item active" } else { "nav-item" },
                        onclick: move |_| state.write().current_view = ActiveView::Dashboard,
                        IconActivity { size: 16 }
                        "Dashboard"
                    }

                    div {
                        class: if app.current_view == ActiveView::Routing { "nav-item active" } else { "nav-item" },
                        onclick: move |_| state.write().current_view = ActiveView::Routing,
                        IconGlobe { size: 16 }
                        "Routing & Egress"
                    }

                    div {
                        class: if app.current_view == ActiveView::Obfuscation { "nav-item active" } else { "nav-item" },
                        onclick: move |_| state.write().current_view = ActiveView::Obfuscation,
                        IconLock { size: 16 }
                        "Stealth & Anti-DPI"
                    }

                    div { class: "nav-label", style: "margin-top: 12px;", "Security & Identity" }

                    div {
                        class: if app.current_view == ActiveView::Identity { "nav-item active" } else { "nav-item" },
                        onclick: move |_| state.write().current_view = ActiveView::Identity,
                        IconShield { size: 16 }
                        "Zero-Trust & ACL"
                    }

                    div {
                        class: if app.current_view == ActiveView::Services { "nav-item active" } else { "nav-item" },
                        onclick: move |_| state.write().current_view = ActiveView::Services,
                        IconServer { size: 16 }
                        "Network Services"
                    }

                    div {
                        class: if app.current_view == ActiveView::Audit { "nav-item active" } else { "nav-item" },
                        onclick: move |_| state.write().current_view = ActiveView::Audit,
                        IconDatabase { size: 16 }
                        "Merkle Ledger & Debug"
                    }

                    div {
                        class: if app.current_view == ActiveView::Platform { "nav-item active" } else { "nav-item" },
                        onclick: move |_| state.write().current_view = ActiveView::Platform,
                        IconCpu { size: 16 }
                        "Platform & Adapters"
                    }
                }

                div { style: "padding: 12px; background: var(--bg-elevation-1); border-radius: var(--radius-sm); border: 1px solid var(--border-subtle); font-size: 11px; color: var(--text-muted);",
                    div { "Pure-Rust Dioxus Client" }
                    div { style: "font-family: var(--font-mono); color: var(--accent-cyan); margin-top: 2px;", "Actix Web IPC Direct" }
                }
            }

            // Main Content Body
            main { class: "app-content",
                match app.current_view {
                    ActiveView::Dashboard => rsx! { DashboardView { state } },
                    ActiveView::Routing => rsx! { RoutingView { state } },
                    ActiveView::Obfuscation => rsx! { ObfuscationView { state } },
                    ActiveView::Identity => rsx! { IdentityView { state } },
                    ActiveView::Services => rsx! { ServicesView { state } },
                    ActiveView::Audit => rsx! { AuditView { state } },
                    ActiveView::Platform => rsx! { PlatformView { state } },
                }
            }

            // Mobile Bottom Navigation Bar
            nav { class: "mobile-nav",
                div {
                    class: if app.current_view == ActiveView::Dashboard { "mobile-nav-item active" } else { "mobile-nav-item" },
                    onclick: move |_| state.write().current_view = ActiveView::Dashboard,
                    IconActivity { size: 18 }
                    span { "Status" }
                }
                div {
                    class: if app.current_view == ActiveView::Routing { "mobile-nav-item active" } else { "mobile-nav-item" },
                    onclick: move |_| state.write().current_view = ActiveView::Routing,
                    IconGlobe { size: 18 }
                    span { "Routing" }
                }
                div {
                    class: if app.current_view == ActiveView::Obfuscation { "mobile-nav-item active" } else { "mobile-nav-item" },
                    onclick: move |_| state.write().current_view = ActiveView::Obfuscation,
                    IconLock { size: 18 }
                    span { "Stealth" }
                }
                div {
                    class: if app.current_view == ActiveView::Identity { "mobile-nav-item active" } else { "mobile-nav-item" },
                    onclick: move |_| state.write().current_view = ActiveView::Identity,
                    IconShield { size: 18 }
                    span { "Access" }
                }
                div {
                    class: if app.current_view == ActiveView::Services { "mobile-nav-item active" } else { "mobile-nav-item" },
                    onclick: move |_| state.write().current_view = ActiveView::Services,
                    IconServer { size: 18 }
                    span { "Services" }
                }
            }
        }
    }
}
