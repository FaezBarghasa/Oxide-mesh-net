//! Design token definitions and embedded CSS constants for oxide-ui

pub const EMBEDDED_CSS: &str = include_str!("style.css");

pub struct DesignTokens;

impl DesignTokens {
    pub const BG_CANVAS: &'static str = "var(--bg-canvas)";
    pub const BG_SURFACE: &'static str = "var(--bg-surface)";
    pub const BG_ELEVATION_1: &'static str = "var(--bg-elevation-1)";
    pub const BG_ELEVATION_2: &'static str = "var(--bg-elevation-2)";
    pub const BG_ELEVATION_3: &'static str = "var(--bg-elevation-3)";

    pub const TEXT_PRIMARY: &'static str = "var(--text-primary)";
    pub const TEXT_SECONDARY: &'static str = "var(--text-secondary)";
    pub const TEXT_MUTED: &'static str = "var(--text-muted)";

    pub const ACCENT_CYAN: &'static str = "var(--accent-cyan)";
    pub const ACCENT_EMERALD: &'static str = "var(--accent-emerald)";
    pub const ACCENT_AMBER: &'static str = "var(--accent-amber)";
    pub const ACCENT_ROSE: &'static str = "var(--accent-rose)";
    pub const ACCENT_VIOLET: &'static str = "var(--accent-violet)";

    pub const BORDER_SUBTLE: &'static str = "var(--border-subtle)";
    pub const BORDER_STRONG: &'static str = "var(--border-strong)";
    pub const BORDER_FOCUS: &'static str = "var(--border-focus)";
}
