//! Non-intrusive peripheral notification toasts

use dioxus::prelude::*;
use crate::components::icons::{IconAlert, IconCheck, IconShield};

#[derive(Clone, PartialEq, Eq)]
pub enum ToastType {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Clone, PartialEq, Eq)]
pub struct ToastItem {
    pub id: u64,
    pub title: String,
    pub message: String,
    pub r#type: ToastType,
}

#[component]
pub fn ToastContainer(toasts: Vec<ToastItem>) -> Element {
    if toasts.is_empty() {
        return rsx! { div {} };
    }

    rsx! {
        div { class: "toast-container",
            for t in toasts {
                div { class: "toast", key: "{t.id}",
                    match t.r#type {
                        ToastType::Info => rsx! { IconShield { size: 16, color: "var(--accent-cyan)" } },
                        ToastType::Success => rsx! { IconCheck { size: 16, color: "var(--accent-emerald)" } },
                        ToastType::Warning => rsx! { IconAlert { size: 16, color: "var(--accent-amber)" } },
                        ToastType::Error => rsx! { IconAlert { size: 16, color: "var(--accent-rose)" } },
                    }
                    div { style: "display: flex; flex-direction: column; gap: 2px;",
                        span { style: "font-weight: 600; font-size: 13px; color: var(--text-primary);", "{t.title}" }
                        span { style: "font-size: 12px; color: var(--text-secondary);", "{t.message}" }
                    }
                }
            }
        }
    }
}
