//! Byte-pair (**digraph**) histograms for binary visualization: map each pair
//! `(b[i], b[i+1])` to a 256×256 grid and count occurrences. Cells are keyed as
//! [`Digraph::get`](crate::Digraph::get)`(first, second)` (first byte of the pair, then second).
//! Heatmaps are drawn **CantorDust-style**: image column = second byte, row = first byte
//! (same convention as Ghidra CantorDust two-tuple view). [`render_ascii`](crate::render_ascii)
//! draws a downsampled terminal heatmap (tone via [`AsciiParams::ramp`](crate::AsciiParams));
//! PNG/SVG use [`HeatmapPalette`](crate::HeatmapPalette); [`render_rgba_pixels`](crate::render_rgba_pixels)
//! builds a raw [`RgbaPixmap`](crate::RgbaPixmap) without the `image` crate. Enable `image` / `svg` for PNG/SVG.
//!
//! # Example
//!
//! Read a file and write a PNG heatmap (`image` feature required for [`Digraph::to_png`](Digraph::to_png)):
//!
//! ```ignore
//! use digraph::{Digraph, Mode, RenderParams};
//! use std::fs::File;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let bytes = std::fs::read("input.bin")?;
//!     let d = Digraph::from_bytes_with_mode(&bytes, Mode::Overlapping);
//!     d.to_png(RenderParams::default(), File::create("digraph.png")?)?;
//!     Ok(())
//! }
//! ```
//!
//! # Render API Guide
//!
//! - `Digraph::to_rgba_pixels` / [`render_rgba_pixels`](crate::render_rgba_pixels):
//!   returns a crate-native [`RgbaPixmap`](crate::RgbaPixmap) and requires no features.
//! - `Digraph::to_rgba` / [`render_rgba`](crate::render_rgba): returns
//!   `image::RgbaImage` (`feature = "image"`).
//! - `Digraph::to_png` / [`render_png`](crate::render_png): writes PNG
//!   bytes to any `Write + Seek` sink (`feature = "image"`).
//! - `Digraph::to_svg_heatmap` / [`render_svg_heatmap`](crate::render_svg_heatmap):
//!   returns an SVG document string (`feature = "svg"`).
//!
//! ASCII and raster defaults use [`Scale::CantorDust`](crate::Scale::CantorDust),
//! while SVG defaults to [`Scale::Log1p`](crate::Scale::Log1p) for denser
//! documents. Set `scale` explicitly in params if you need cross-output consistency.

mod accumulate;
#[cfg(feature = "image")]
mod error;
mod digraph;
mod normalize;
mod palette;
mod render;

pub use digraph::{Digraph, DigraphBuilder, Mode};
pub use normalize::Scale;
pub use palette::HeatmapPalette;
pub use render::{render_ascii, AsciiParams, render_rgba_pixels, RgbaPixmap, RenderParams};

#[cfg(feature = "image")]
pub use error::RenderError;
#[cfg(feature = "image")]
pub use render::{render_png, render_rgba};

#[cfg(feature = "svg")]
pub use render::{render_svg_heatmap, SvgParams};
