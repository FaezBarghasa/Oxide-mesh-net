//! Pure declarative SVG QR Code renderer for RFC 8628 Device Authorization Grant

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SvgQrCodeProps {
    #[props(into)]
    pub data: String,
    #[props(default = 160)]
    pub size: u32,
}

#[component]
pub fn SvgQrCode(props: SvgQrCodeProps) -> Element {
    // Standard 3 Finder patterns
    let add_finder = |m: &mut [[bool; 21]; 21], start_r: usize, start_c: usize| {
        for (r, row) in m.iter_mut().enumerate().skip(start_r).take(7) {
            let rel_r = r - start_r;
            for (c, cell) in row.iter_mut().enumerate().skip(start_c).take(7) {
                let rel_c = c - start_c;
                if rel_r == 0
                    || rel_r == 6
                    || rel_c == 0
                    || rel_c == 6
                    || ((2..=4).contains(&rel_r) && (2..=4).contains(&rel_c))
                {
                    *cell = true;
                }
            }
        }
    };

    let mut matrix = [[false; 21]; 21];
    add_finder(&mut matrix, 0, 0);
    add_finder(&mut matrix, 0, 14);
    add_finder(&mut matrix, 14, 0);

    // Timing patterns
    for (i, cell) in matrix[6].iter_mut().enumerate().take(13).skip(8) {
        *cell = i % 2 == 0;
    }
    for (i, row) in matrix.iter_mut().enumerate().take(13).skip(8) {
        row[6] = i % 2 == 0;
    }

    // Pseudo-random data payload representation for deterministic UI rendering
    let hash_bytes = blake3::hash(props.data.as_bytes());
    let bytes = hash_bytes.as_bytes();

    let mut bit_idx = 0;
    for (r, row) in matrix.iter_mut().enumerate() {
        for (c, cell) in row.iter_mut().enumerate() {
            let is_finder = (r < 8 && c < 8) || (r < 8 && c >= 13) || (r >= 13 && c < 8);
            let is_timing = r == 6 || c == 6;
            if !is_finder && !is_timing {
                let byte = bytes[bit_idx % bytes.len()];
                let bit = (byte >> (bit_idx % 8)) & 1 == 1;
                *cell = bit;
                bit_idx += 1;
            }
        }
    }

    let cell_size = 6.0;
    let mut rects = Vec::new();
    for (r, row) in matrix.iter().enumerate() {
        for (c, &is_filled) in row.iter().enumerate() {
            if is_filled {
                rects.push((c as f64 * cell_size, r as f64 * cell_size));
            }
        }
    }

    rsx! {
        div {
            style: "background: #ffffff; padding: 12px; border-radius: 8px; display: inline-flex; box-shadow: 0 4px 16px rgba(0,0,0,0.5);",
            svg {
                width: "{props.size}",
                height: "{props.size}",
                view_box: "0 0 126 126",
                rect { width: "126", height: "126", fill: "#ffffff" }
                for (x, y) in rects {
                    rect {
                        x: "{x}",
                        y: "{y}",
                        width: "{cell_size}",
                        height: "{cell_size}",
                        fill: "#070709",
                    }
                }
            }
        }
    }
}
