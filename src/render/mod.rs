//! Render targets: ASCII (always), optional PNG / SVG.

mod ascii;
#[cfg(feature = "image")]
mod image_png;
pub use ascii::{render_ascii, AsciiParams};

#[cfg(feature = "image")]
pub use image_png::{render_png, render_rgba, RenderParams};

#[cfg(feature = "svg")]
mod svg;
#[cfg(feature = "svg")]
pub use svg::{render_svg_heatmap, SvgParams};
