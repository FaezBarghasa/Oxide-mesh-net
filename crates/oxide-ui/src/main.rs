//! oxide-ui application entry point
//!
//! Multiplatform reactive client interface (Desktop, Mobile, and Web) for Oxide Mesh Net.

use oxide_ui::App;

fn main() {
    #[cfg(feature = "desktop")]
    {
        use dioxus::desktop::{Config, WindowBuilder};
        let cfg = Config::new().with_window(
            WindowBuilder::new()
                .with_title("Oxide Mesh Net - Pure-Rust Zero-Trust Node")
                .with_inner_size(dioxus::desktop::tao::dpi::LogicalSize::new(1280.0, 800.0))
                .with_min_inner_size(dioxus::desktop::tao::dpi::LogicalSize::new(360.0, 600.0)),
        );
        dioxus::LaunchBuilder::desktop().with_cfg(cfg).launch(App);
    }

    #[cfg(not(feature = "desktop"))]
    {
        dioxus::launch(App);
    }
}
