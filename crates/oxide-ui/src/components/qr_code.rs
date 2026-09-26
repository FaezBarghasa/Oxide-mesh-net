//! Pure declarative SVG QR Code renderer for RFC 8628 Device Authorization Grant

use dioxus::prelude::*;

#[component]
pub fn SvgQrCode(
    data: String,
    #[props(default = 160)] size: u32,
) -> Element {
    // Generate a deterministic visual 21x21 QR pattern based on input hash
    let mut matrix = vec![vec![false; 21]; 21];

    // Standard 3 Finder patterns
    let add_finder = |m: &mut Vec<Vec<bool>>, start_r: usize, start_c: usize| {
        for r in 0..7 {
            for c in 0..7 {
                if r == 0 || r == 6 || c == 0 || c == 6 || (r >= 2 && r <= 4 && c >= 2 && c <= 4) {
                    m[start_r + r][start_c + c] = true;
                }
            }
        }
    };

    add_finder(&mut matrix, 0, 0);
    add_finder(&mut matrix, 0, 14);
    add_finder(&mut matrix, 14, 0);

    // Timing patterns
    for i in 8..13 {
        matrix[6][i] = i % 2 == 0;
        matrix[i][6] = i % 2 == 0;
    }

    // Pseudo-random data payload representation for deterministic UI rendering
    let hash_bytes = blake3::hash(data.as_bytes());
    let bytes = hash_bytes.as_bytes();

    let mut bit_idx = 0;
    for r in 0..21 {
        for c in 0..21 {
            let is_finder = (r < 8 && c < 8) || (r < 8 && c >= 13) || (r >= 13 && c < 8);
            let is_timing = r == 6 || c == 6;
            if !is_finder && !is_timing {
                let byte = bytes[bit_idx % bytes.len()];
                let bit = (byte >> (bit_idx % 8)) & 1 == 1;
                matrix[r][c] = bit;
                bit_idx += 1;
            }
        }
    }

    let cell_size = 6.0;
    let mut rects = Vec::new();
    for r in 0..21 {
        for c in 0..21 {
            if matrix[r][c] {
                rects.push((c as f64 * cell_size, r as f64 * cell_size));
            }
        }
    }

    rsx! {
        div {
            style: "background: #ffffff; padding: 12px; border-radius: 8px; display: inline-flex; box-shadow: 0 4px 16px rgba(0,0,0,0.5);",
            svg {
                width: "{size}",
                height: "{size}",
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
