//! Square RGBA heatmap raster (no `image` crate).

use crate::digraph::Digraph;
use crate::normalize::Scale;
use crate::palette::HeatmapPalette;

/// Controls upscaling and color mapping for raster output (PNG, pixmap, GUI).
#[derive(Clone, Copy, Debug)]
pub struct RenderParams {
    /// Pixels per axis cell (image side = 256 * cell_pixels).
    pub cell_pixels: u32,
    pub scale: Scale,
    pub palette: HeatmapPalette,
}

impl Default for RenderParams {
    fn default() -> Self {
        Self {
            cell_pixels: 2,
            scale: Scale::Log1p,
            palette: HeatmapPalette::default(),
        }
    }
}

/// Row-major RGBA8 buffer (`rgba[(y * width + x) * 4 + c]`); top row is small **first
/// byte** of each pair (CantorDust-style: column = second byte, row = first byte).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RgbaPixmap {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

impl RgbaPixmap {
    /// Returns `true` if `rgba.len() == width * height * 4`.
    pub fn is_valid(&self) -> bool {
        self.rgba.len() as u64 == (self.width as u64) * (self.height as u64) * 4
    }
}

/// Renders the digraph heatmap into an RGBA8 pixmap.
pub fn render_rgba_pixels(digraph: &Digraph, params: RenderParams) -> RgbaPixmap {
    let cell = params.cell_pixels.max(1) as usize;
    let side = 256 * cell;
    let w = side as u32;
    let h = side as u32;
    let stride = side * 4;
    let mut rgba = vec![0u8; side * side * 4];
    let max = digraph.max_count();
    let clip = params.scale.clip_high(digraph);

    // CantorDust `TwoTupleVisualizer`: pixel at (second_byte, first_byte) on screen.
    for first in 0u16..256 {
        for second in 0u16..256 {
            let v = digraph.get(first as u8, second as u8);
            let t = params.scale.map(v, max, clip);
            let [r, g, b, a] = params.palette.rgba(t);
            let xi = second as usize * cell;
            let yi = first as usize * cell;
            for dy in 0..cell {
                for dx in 0..cell {
                    let row = yi + dy;
                    let col = xi + dx;
                    let idx = row * stride + col * 4;
                    rgba[idx] = r;
                    rgba[idx + 1] = g;
                    rgba[idx + 2] = b;
                    rgba[idx + 3] = a;
                }
            }
        }
    }

    RgbaPixmap {
        width: w,
        height: h,
        rgba,
    }
}

#[cfg(test)]
mod tests {
    use super::{render_rgba_pixels, RenderParams};
    use crate::digraph::{Digraph, Mode};
    use crate::normalize::Scale;
    use crate::palette::HeatmapPalette;

    #[test]
    fn rgba_pixmap_matches_cell_size() {
        let d = Digraph::from_bytes_with_mode(&[0, 255], Mode::Overlapping);
        let p = render_rgba_pixels(
            &d,
            RenderParams {
                cell_pixels: 2,
                scale: Scale::Linear,
                palette: HeatmapPalette::Magma,
            },
        );
        assert_eq!(p.width, 512);
        assert_eq!(p.height, 512);
        assert!(p.is_valid());
        let stride = 512 * 4;
        let yi = 0 * 2;
        let xi = 255 * 2;
        let idx = yi * stride + xi * 4;
        assert!(p.rgba[idx + 3] > 0);
        assert!(p.rgba[idx + 1] > 10 || p.rgba[idx + 0] > 0);
    }
}
