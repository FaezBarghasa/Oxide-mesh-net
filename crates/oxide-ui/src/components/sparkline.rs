//! High-frequency low-overhead SVG micro-sparkline renderer for rolling throughput & latency

use dioxus::prelude::*;

#[component]
pub fn Sparkline(
    points: Vec<f64>,
    #[props(default = "#00f0ff")] stroke_color: &'static str,
    #[props(default = "rgba(0, 240, 255, 0.15)")] fill_color: &'static str,
    #[props(default = 38)] height: u32,
) -> Element {
    if points.is_empty() {
        return rsx! { div { class: "sparkline-container" } };
    }

    let min_val = points.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_val = points.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = if (max_val - min_val).abs() < 1e-6 { 1.0 } else { max_val - min_val };

    let count = points.len();
    let width = 200.0;
    let h = height as f64;

    let mut path_data = String::new();
    let mut area_data = String::new();

    for (i, &pt) in points.iter().enumerate() {
        let x = if count > 1 { (i as f64 / (count - 1) as f64) * width } else { 0.0 };
        let normalized = (pt - min_val) / range;
        let y = h - (normalized * (h - 6.0) + 3.0);

        if i == 0 {
            path_data.push_str(&format!("M {:.1} {:.1}", x, y));
            area_data.push_str(&format!("M {:.1} {:.1} L {:.1} {:.1}", x, h, x, y));
        } else {
            path_data.push_str(&format!(" L {:.1} {:.1}", x, y));
            area_data.push_str(&format!(" L {:.1} {:.1}", x, y));
        }
    }

    if count > 1 {
        area_data.push_str(&format!(" L {:.1} {:.1} Z", width, h));
    }

    rsx! {
        div { class: "sparkline-container",
            svg {
                class: "sparkline-svg",
                view_box: "0 0 200 {height}",
                preserve_aspect_ratio: "none",
                path {
                    d: "{area_data}",
                    fill: "{fill_color}",
                    stroke: "none",
                }
                path {
                    d: "{path_data}",
                    fill: "none",
                    stroke: "{stroke_color}",
                    stroke_width: "1.5",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
            }
        }
    }
}
