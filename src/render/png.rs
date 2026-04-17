use crate::digraph::Digraph;
use crate::error::RenderError;
use crate::render::raster::{render_rgba_pixels, RenderParams};
use image::{ImageBuffer, RgbaImage};

/// RGBA image buffer (`width` = `height` = 256 * cell_pixels).
pub fn render_rgba(digraph: &Digraph, params: RenderParams) -> RgbaImage {
    let pm = render_rgba_pixels(digraph, params);
    ImageBuffer::from_raw(pm.width, pm.height, pm.rgba).expect("valid pixmap")
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
