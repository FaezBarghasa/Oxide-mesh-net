//! Range slider with live value tooltip

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SliderProps {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    #[props(default)]
    pub step: Option<f64>,
    #[props(default)]
    pub unit: Option<String>,
    pub on_change: EventHandler<f64>,
}

#[component]
pub fn Slider(props: SliderProps) -> Element {
    let s = props.step.unwrap_or(1.0);
    let u = props.unit.clone().unwrap_or_default();
    let val = props.value;

    rsx! {
        div { style: "display: flex; align-items: center; gap: 12px; width: 100%;",
            input {
                r#type: "range",
                min: "{props.min}",
                max: "{props.max}",
                step: "{s}",
                value: "{val}",
                style: "flex: 1; accent-color: var(--accent-cyan); height: 6px; cursor: pointer;",
                oninput: move |evt| {
                    if let Ok(v) = evt.value().parse::<f64>() {
                        props.on_change.call(v);
                    }
                }
            }
            span {
                class: "badge badge-muted",
                style: "min-width: 54px; text-align: center; font-family: var(--font-mono);",
                "{val} {u}"
            }
        }
    }
}
