//! Semantic badges for status indicators

use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BadgeVariant {
    Healthy,
    Warning,
    Critical,
    Cyan,
    Violet,
    Muted,
}

#[component]
pub fn Badge(
    variant: BadgeVariant,
    children: Element,
) -> Element {
    let class_name = match variant {
        BadgeVariant::Healthy => "badge badge-healthy",
        BadgeVariant::Warning => "badge badge-warning",
        BadgeVariant::Critical => "badge badge-critical",
        BadgeVariant::Cyan => "badge badge-cyan",
        BadgeVariant::Violet => "badge badge-violet",
        BadgeVariant::Muted => "badge badge-muted",
    };

    rsx! {
        span { class: "{class_name}",
            {children}
        }
    }
}
