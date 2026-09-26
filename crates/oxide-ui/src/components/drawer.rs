//! Diagnostic slide-out drawer with backdrop blur

use dioxus::prelude::*;
use crate::components::icons::IconClose;

#[component]
pub fn Drawer(
    is_open: bool,
    title: String,
    on_close: EventHandler<()>,
    children: Element,
) -> Element {
    if !is_open {
        return rsx! { div {} };
    }

    rsx! {
        div {
            class: "drawer-overlay",
            onclick: move |_| on_close.call(()),
            div {
                class: "drawer",
                onclick: |evt| evt.stop_propagation(),
                div {
                    style: "display: flex; align-items: center; justify-content: space-between; border-bottom: 1px solid var(--border-subtle); padding-bottom: 14px;",
                    h3 { style: "font-size: 16px; font-weight: 700; letter-spacing: -0.01em;", "{title}" }
                    button {
                        class: "btn btn-ghost",
                        style: "padding: 4px; border-radius: 50%;",
                        onclick: move |_| on_close.call(()),
                        IconClose { size: 18 }
                    }
                }
                div { style: "display: flex; flex-direction: column; gap: 14px; margin-top: 8px;",
                    {children}
                }
            }
        }
    }
}
