//! Action button component with variants and loading state

use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
    Ghost,
}

#[component]
pub fn Button(
    #[props(default = ButtonVariant::Secondary)] variant: ButtonVariant,
    #[props(default = false)] disabled: bool,
    #[props(default = false)] loading: bool,
    on_click: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let variant_class = match variant {
        ButtonVariant::Primary => "btn btn-primary",
        ButtonVariant::Secondary => "btn btn-secondary",
        ButtonVariant::Danger => "btn btn-danger",
        ButtonVariant::Ghost => "btn btn-ghost",
    };

    let is_disabled = disabled || loading;

    rsx! {
        button {
            class: "{variant_class}",
            disabled: is_disabled,
            onclick: move |evt| {
                if !is_disabled {
                    if let Some(ref handler) = on_click {
                        handler.call(evt);
                    }
                }
            },
            if loading {
                span { class: "spinner", style: "display: inline-block; width: 14px; height: 14px; border: 2px solid currentColor; border-top-color: transparent; border-radius: 50%; animation: spin 0.6s linear infinite;" }
            }
            {children}
        }
    }
}
