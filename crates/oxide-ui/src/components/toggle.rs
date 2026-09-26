//! Switch / Toggle component

use dioxus::prelude::*;

#[component]
pub fn Toggle(
    checked: bool,
    disabled: Option<bool>,
    on_toggle: EventHandler<bool>,
) -> Element {
    let is_disabled = disabled.unwrap_or(false);

    rsx! {
        label { class: "switch",
            input {
                r#type: "checkbox",
                checked: "{checked}",
                disabled: is_disabled,
                onchange: move |evt| {
                    if !is_disabled {
                        on_toggle.call(evt.value() == "true");
                    }
                }
            }
            span { class: "slider-round" }
        }
    }
}
