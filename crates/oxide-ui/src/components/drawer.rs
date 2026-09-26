//! Diagnostic slide-out drawer with backdrop blur

use dioxus::prelude::*;
use crate::components::icons::IconClose;

#[derive(Props, Clone, PartialEq)]
pub struct DrawerProps {
    pub is_open: bool,
    #[props(into)]
    pub title: String,
    pub on_close: EventHandler<()>,
    pub children: Element,
}

#[component]
pub fn Drawer(props: DrawerProps) -> Element {
    if !props.is_open {
        return rsx! { div {} };
    }

    rsx! {
        div {
            class: "drawer-overlay",
            onclick: move |_| props.on_close.call(()),
            div {
                class: "drawer",
                onclick: |evt| evt.stop_propagation(),
                div {
                    style: "display: flex; align-items: center; justify-content: space-between; border-bottom: 1px solid var(--border-subtle); padding-bottom: 14px;",
                    h3 { style: "font-size: 16px; font-weight: 700; letter-spacing: -0.01em;", "{props.title}" }
                    button {
                        class: "btn btn-ghost",
                        style: "padding: 4px; border-radius: 50%;",
                        onclick: move |_| props.on_close.call(()),
                        IconClose { size: 18 }
                    }
                }
                div { style: "display: flex; flex-direction: column; gap: 14px; margin-top: 8px;",
                    {props.children}
                }
            }
        }
    }
}
