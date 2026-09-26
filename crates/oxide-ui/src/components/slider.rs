//! Range slider with live value tooltip

use dioxus::prelude::*;

#[component]
pub fn Slider(
    value: f64,
    min: f64,
    max: f64,
    step: Option<f64>,
    unit: Option<String>,
    on_change: EventHandler<f64>,
) -> Element {
    let s = step.unwrap_or(1.0);
    let u = unit.unwrap_or_default();

    rsx! {
        div { style: "display: flex; align-items: center; gap: 12px; width: 100%;",
            input {
                r#type: "range",
                min: "{min}",
                max: "{max}",
                step: "{s}",
                value: "{value}",
                style: "flex: 1; accent-color: var(--accent-cyan); height: 6px; cursor: pointer;",
                oninput: move |evt| {
                    if let Ok(v) = evt.value().parse::<f64>() {
                        on_change.call(v);
                    }
                }
            }
            span {
                class: "badge badge-muted",
                style: "min-width: 54px; text-align: center; font-family: var(--font-mono);",
                "{value} {u}"
            }
        }
    }
}
