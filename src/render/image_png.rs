use crate::digraph::Digraph;
use crate::error::RenderError;
use crate::normalize::Scale;
use crate::palette::HeatmapPalette;
use image::{ImageBuffer, Rgba, RgbaImage};

/// Controls upscaling and color mapping for raster output.
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

/// RGBA image buffer (`width` = `height` = 256 * cell_pixels).
pub fn render_rgba(digraph: &Digraph, params: RenderParams) -> RgbaImage {
    let cell = params.cell_pixels.max(1) as usize;
    let side = 256 * cell;
    let max = digraph.max_count();
    let clip = params.scale.clip_high(digraph);

    let mut img: RgbaImage = ImageBuffer::new(side as u32, side as u32);
    for y in 0u16..256 {
        for x in 0u16..256 {
            let v = digraph.get(x as u8, y as u8);
            let t = params.scale.map(v, max, clip);
            let px = params.palette.rgba(t);
            let rgba = Rgba(px);
            let xi = x as usize * cell;
            let yi = y as usize * cell;
            for dy in 0..cell {
                for dx in 0..cell {
                    img.put_pixel((xi + dx) as u32, (yi + dy) as u32, rgba);
                }
            }
        }
    }
    img
}

/// Writes a PNG to any `Write` + `Seek` sink (e.g. `std::fs::File` or `std::io::Cursor<Vec<u8>>`).
pub fn render_png<W: std::io::Write + std::io::Seek>(
    digraph: &Digraph,
    params: RenderParams,
    writer: W,
) -> Result<(), RenderError> {
    let img = render_rgba(digraph, params);
    img.write_to(
        &mut std::io::BufWriter::new(writer),
        image::ImageFormat::Png,
    )?;
    Ok(())
}
