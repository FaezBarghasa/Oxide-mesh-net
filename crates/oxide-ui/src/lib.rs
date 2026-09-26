//! oxide-ui: Pure-Rust Cross-Platform Reactive Client Interface

pub mod app;
pub mod components;
pub mod models;
pub mod state;
pub mod theme;
pub mod views;

pub use app::App;
pub use models::*;
pub use state::AppState;
pub use theme::EMBEDDED_CSS;
