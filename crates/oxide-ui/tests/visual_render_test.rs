//! Headless rendering & view state verification for oxide-ui

use dioxus::prelude::*;
use oxide_ui::app::App;
use oxide_ui::state::{ActiveView, AppState};

#[test]
fn test_app_state_initialization() {
    let state = AppState::default();
    assert_eq!(state.current_view, ActiveView::Dashboard);
    assert_eq!(state.peers.len(), 3);
    assert_eq!(state.routes.len(), 3);
    assert_eq!(state.telemetry.overlay_ipv4, "100.64.0.42");
}

#[test]
fn test_all_views_navigation_transitions() {
    let mut state = AppState::default();
    let views = [
        ActiveView::Dashboard,
        ActiveView::Routing,
        ActiveView::Obfuscation,
        ActiveView::Identity,
        ActiveView::Services,
        ActiveView::Audit,
        ActiveView::Platform,
    ];

    for view in views {
        state.current_view = view;
        assert_eq!(state.current_view, view);
    }
}

#[test]
fn test_embedded_css_design_tokens() {
    let css = oxide_ui::EMBEDDED_CSS;
    assert!(css.contains("--bg-canvas: #070709;"));
    assert!(css.contains("--accent-cyan: #00f0ff;"));
    assert!(css.contains("--accent-emerald: #10b981;"));
    assert!(css.contains(".app-shell"));
    assert!(css.contains(".app-sidebar"));
    assert!(css.contains(".mobile-nav"));
    assert!(css.contains("@media (max-width: 768px)"));
}

#[test]
fn test_headless_app_dom_render() {
    let mut dom = VirtualDom::new(App);
    dom.rebuild_in_place();

    let rendered = dioxus_ssr::render(&dom);
    assert!(rendered.contains("OXIDE MESH NET"));
    assert!(rendered.contains("DAEMON SYNCED"));
    assert!(rendered.contains("100.64.0.42"));
    assert!(rendered.contains("Dashboard"));
    assert!(rendered.contains("Routing &amp; Egress"));
    assert!(rendered.contains("Stealth &amp; Anti-DPI"));
    assert!(rendered.contains("mobile-nav"));
}
