//! Downsampled ASCII heatmap (terminal-friendly).

use crate::digraph::Digraph;
use crate::normalize::Scale;

const MAX_DIM: u32 = 256;

/// Grid size and tone mapping for [`render_ascii`].
#[derive(Clone, Debug, PartialEq)]
pub struct AsciiParams {
    /// Output columns (each column aggregates a vertical slice of X).
    pub cols: u32,
    /// Output rows (each row aggregates a horizontal slice of Y, top row is small Y).
    pub rows: u32,
    pub scale: Scale,
    /// Characters from dim to bright; must be non-empty when UTF-8 graphemes are counted.
    pub ramp: String,
}

impl Default for AsciiParams {
    fn default() -> Self {
        Self {
            cols: 80,
            rows: 36,
            scale: Scale::Log1p,
            ramp: " .:-=+*#%@".to_string(),
        }
    }
}

fn clamp_dim(n: u32) -> u32 {
    n.clamp(1, MAX_DIM)
}

fn tile_max(digraph: &Digraph, x0: u32, x1: u32, y0: u32, y1: u32) -> u32 {
    let mut m = 0u32;
    for x in x0..x1 {
        for y in y0..y1 {
            m = m.max(digraph.get(x as u8, y as u8));
        }
    }
    m
}

/// Renders a downsampled digraph as newline-terminated rows of ASCII/Unicode characters.
pub fn render_ascii(digraph: &Digraph, params: AsciiParams) -> String {
    let cols = clamp_dim(params.cols);
    let rows = clamp_dim(params.rows);
    let ramp_vec: Vec<char> = params.ramp.chars().collect();
    let default_ramp = [' ', '#'];
    let ramp: &[char] = if ramp_vec.is_empty() {
        default_ramp.as_slice()
    } else {
        ramp_vec.as_slice()
    };

    let max = digraph.max_count();
    let clip = params.scale.clip_high(digraph);
    let last = ramp.len() - 1;

    let mut out = String::with_capacity((cols as usize + 1) * rows as usize);
    for row in 0..rows {
        let y0 = row * 256 / rows;
        let y1 = (row + 1) * 256 / rows;
        for col in 0..cols {
            let x0 = col * 256 / cols;
            let x1 = (col + 1) * 256 / cols;
            let v = tile_max(digraph, x0, x1, y0, y1);
            let ch = if max == 0 || v == 0 {
                ramp[0]
            } else {
                let t = params.scale.map(v, max, clip);
                let idx = ((t * last as f32).round() as usize).min(last);
                ramp[idx]
            };
            out.push(ch);
        }
        out.push('\n');
    }
    out
}
