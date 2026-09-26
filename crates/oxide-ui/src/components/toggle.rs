//! Switch / Toggle component

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToggleProps {
    pub checked: bool,
    #[props(default)]
    pub disabled: Option<bool>,
    pub on_toggle: EventHandler<bool>,
}

#[component]
pub fn Toggle(props: ToggleProps) -> Element {
    let is_disabled = props.disabled.unwrap_or(false);

    rsx! {
        label { class: "switch",
            input {
                r#type: "checkbox",
                checked: "{props.checked}",
                disabled: is_disabled,
                onchange: move |evt| {
                    if !is_disabled {
                        props.on_toggle.call(evt.value() == "true");
                    }
                }
            }
            span { class: "slider-round" }
        }
    }
}
