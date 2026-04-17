//! Byte-pair (**digraph**) histograms for binary visualization: map each pair
//! `(b[i], b[i+1])` to a 256×256 grid and count occurrences. Cells are keyed as
//! [`Digraph::get`](crate::Digraph::get)`(first, second)` (first byte of the pair, then second).
//! Heatmaps are drawn **CantorDust-style**: image column = second byte, row = first byte
//! (same convention as Ghidra CantorDust two-tuple view). [`render::render_ascii`](crate::render::render_ascii)
//! draws a downsampled terminal heatmap (tone via [`AsciiParams::ramp`](crate::render::AsciiParams));
//! PNG/SVG use [`HeatmapPalette`](crate::HeatmapPalette); [`render::render_rgba_pixels`](crate::render::render_rgba_pixels)
//! builds a raw [`RgbaPixmap`](crate::render::RgbaPixmap) without the `image` crate. Enable `image` / `svg` for PNG/SVG.
//!
//! # Example
//!
//! ```
//! use digraph::{Digraph, DigraphBuilder, Mode};
//!
//! let d = Digraph::from_bytes_with_mode(&[0, 1, 2], Mode::Overlapping);
//! assert_eq!(d.get(0, 1), 1);
//! assert_eq!(d.get(1, 2), 1);
//!
//! let mut b = DigraphBuilder::new(Mode::Overlapping);
//! b.push_bytes(&[0, 1]);
//! b.push_bytes(&[1, 2]);
//! let d2 = b.finish();
//! assert_eq!(d2.get(0, 1), 1);
//! assert_eq!(d2.get(1, 2), 1);
//! ```

mod accumulate;
#[cfg(feature = "image")]
mod error;

pub mod digraph;
pub mod normalize;
pub mod palette;

pub use palette::HeatmapPalette;

pub mod render;

#[cfg(feature = "image")]
pub use error::RenderError;

pub use digraph::{Digraph, DigraphBuilder, Mode};
