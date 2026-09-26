//! Card and surface container components

use dioxus::prelude::*;

#[component]
pub fn Card(
    title: Option<String>,
    header_action: Option<Element>,
    children: Element,
) -> Element {
    rsx! {
        div { class: "card",
            if title.is_some() || header_action.is_some() {
                div { class: "card-header",
                    if let Some(t) = title {
                        div { class: "card-title", "{t}" }
                    }
                    if let Some(act) = header_action {
                        {act}
                    }
                }
            }
            {children}
        }
    }
}
