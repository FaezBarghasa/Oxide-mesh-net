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

#[derive(Props, Clone, PartialEq)]
pub struct BadgeProps {
    pub variant: BadgeVariant,
    pub children: Element,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let class_name = match props.variant {
        BadgeVariant::Healthy => "badge badge-healthy",
        BadgeVariant::Warning => "badge badge-warning",
        BadgeVariant::Critical => "badge badge-critical",
        BadgeVariant::Cyan => "badge badge-cyan",
        BadgeVariant::Violet => "badge badge-violet",
        BadgeVariant::Muted => "badge badge-muted",
    };

    rsx! {
        span { class: "{class_name}",
            {props.children}
        }
    }
}
