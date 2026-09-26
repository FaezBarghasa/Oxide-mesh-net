//! Action button component with variants and loading state

use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
    Ghost,
}

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    #[props(default = ButtonVariant::Secondary)]
    pub variant: ButtonVariant,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default = false)]
    pub loading: bool,
    #[props(default)]
    pub on_click: Option<EventHandler<MouseEvent>>,
    pub children: Element,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let variant_class = match props.variant {
        ButtonVariant::Primary => "btn btn-primary",
        ButtonVariant::Secondary => "btn btn-secondary",
        ButtonVariant::Danger => "btn btn-danger",
        ButtonVariant::Ghost => "btn btn-ghost",
    };

    let is_disabled = props.disabled || props.loading;

    rsx! {
        button {
            class: "{variant_class}",
            disabled: is_disabled,
            onclick: move |evt| {
                if !is_disabled {
                    if let Some(ref handler) = props.on_click {
                        handler.call(evt);
                    }
                }
            },
            if props.loading {
                span { class: "spinner", style: "display: inline-block; width: 14px; height: 14px; border: 2px solid currentColor; border-top-color: transparent; border-radius: 50%; animation: spin 0.6s linear infinite;" }
            }
            {props.children}
        }
    }
}
