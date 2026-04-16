//! Optional render targets (PNG / SVG).

#[cfg(feature = "image")]
mod image_png;
#[cfg(feature = "image")]
pub use image_png::{render_png, render_rgba, RenderParams};

#[cfg(feature = "svg")]
mod svg;
#[cfg(feature = "svg")]
pub use svg::{render_svg_heatmap, SvgParams};
