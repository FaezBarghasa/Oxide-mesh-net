//! Card and surface container components

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    #[props(into, default)]
    pub title: Option<String>,
    #[props(default)]
    pub header_action: Option<Element>,
    pub children: Element,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    rsx! {
        div { class: "card",
            if props.title.is_some() || props.header_action.is_some() {
                div { class: "card-header",
                    if let Some(ref t) = props.title {
                        div { class: "card-title", "{t}" }
                    }
                    if let Some(ref act) = props.header_action {
                        {act}
                    }
                }
            }
            {props.children}
        }
    }
}
