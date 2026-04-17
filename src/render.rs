//! Render targets: ASCII (always), optional PNG / SVG.
//!
//! Raster and SVG use [`crate::palette::HeatmapPalette`] via [`RenderParams`](crate::render::RenderParams) and [`SvgParams`](crate::render::SvgParams).

mod ascii;
mod raster;
#[cfg(feature = "image")]
mod png;
pub use crate::palette::HeatmapPalette;
pub use ascii::{render_ascii, AsciiParams};
pub use raster::{render_rgba_pixels, RgbaPixmap, RenderParams};

#[cfg(feature = "image")]
pub use png::{render_png, render_rgba};

#[cfg(feature = "svg")]
mod svg;
#[cfg(feature = "svg")]
pub use svg::{render_svg_heatmap, SvgParams};
