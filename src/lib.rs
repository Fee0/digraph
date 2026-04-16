//! Byte-pair (**digraph**) histograms for binary visualization: map each pair
//! `(b[i], b[i+1])` to a 256×256 grid (first byte = X, second = Y) and count
//! occurrences. [`render::render_ascii`](crate::render::render_ascii) draws a
//! downsampled terminal heatmap (tone via [`AsciiParams::ramp`](crate::render::AsciiParams));
//! PNG/SVG use [`HeatmapPalette`](crate::HeatmapPalette). Enable `image` / `svg` for those backends.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_sample_bin_yields_pairs() {
        let path =
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/sample.bin");
        let bytes = std::fs::read(&path).expect("tests/data/sample.bin missing");
        let d = Digraph::from_bytes(&bytes);
        assert!(d.max_count() > 0, "fixture should contain repeated bigrams");
    }

    #[test]
    fn format_fixtures_have_magic_and_bigrams() {
        use std::path::PathBuf;

        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data");
        let cases: &[(&str, &[u8])] = &[
            ("pe_like.bin", b"MZ"),
            ("elf_like.bin", b"\x7fELF"),
            ("zip_like.bin", b"PK\x03\x04"),
            ("png_like.bin", b"\x89PNG\r\n\x1a\n"),
            ("jpeg_like.bin", b"\xFF\xD8\xFF"),
            ("gzip_like.bin", b"\x1F\x8B\x08"),
        ];
        for (name, magic) in cases {
            let path = dir.join(name);
            let bytes = std::fs::read(&path).unwrap_or_else(|e| panic!("read {name}: {e}"));
            assert!(
                bytes.starts_with(magic),
                "{name} should start with format magic"
            );
            let d = Digraph::from_bytes(&bytes);
            assert!(d.max_count() > 0, "{name} should contain repeated bigrams");
        }
    }

    #[test]
    fn heatmap_palette_midpoints_differ() {
        let t = 0.5_f32;
        assert_ne!(
            HeatmapPalette::Magma.rgba(t),
            HeatmapPalette::Viridis.rgba(t)
        );
        assert_ne!(HeatmapPalette::Magma.rgba(t), HeatmapPalette::Gray.rgba(t));
    }

    #[test]
    fn matrix_palette_dark_to_neon_green() {
        let dark = HeatmapPalette::Matrix.rgba(0.0);
        let bright = HeatmapPalette::Matrix.rgba(1.0);
        assert!(dark[1] < 32, "dark end should stay low-green");
        assert!(bright[1] > 200, "bright end should be neo green");
        assert!(bright[1] as i32 > bright[0] as i32 * 3);
        assert!(bright[1] as i32 > bright[2] as i32 * 2);
    }

    #[test]
    fn ascii_empty_is_dim() {
        let d = Digraph::empty();
        let s = d.to_ascii(crate::render::AsciiParams {
            cols: 2,
            rows: 2,
            ramp: ".#".to_string(),
            ..crate::render::AsciiParams::default()
        });
        assert_eq!(s, "..\n..\n");
    }

    #[test]
    fn ascii_grid_shape_and_bright_cell() {
        let d = Digraph::from_bytes_with_mode(&[1, 2, 1, 2], Mode::Overlapping);
        let s = d.to_ascii(crate::render::AsciiParams {
            cols: 4,
            rows: 2,
            scale: crate::normalize::Scale::Linear,
            ramp: ".@".to_string(),
        });
        let lines: Vec<&str> = s.lines().collect();
        assert_eq!(lines.len(), 2);
        assert!(lines.iter().all(|line| line.len() == 4));
        assert!(s.contains('@'));
    }

    #[test]
    fn overlapping_pairs() {
        let d = Digraph::from_bytes_with_mode(&[10, 20, 10, 20], Mode::Overlapping);
        assert_eq!(d.get(10, 20), 2);
        assert_eq!(d.get(20, 10), 1);
    }

    #[test]
    fn non_overlapping_pairs() {
        let d = Digraph::from_bytes_with_mode(&[1, 2, 3, 4, 5], Mode::NonOverlapping);
        assert_eq!(d.get(1, 2), 1);
        assert_eq!(d.get(3, 4), 1);
        assert_eq!(d.get(2, 3), 0);
    }

    #[test]
    fn empty_and_singleton() {
        let d = Digraph::from_bytes(&[]);
        assert_eq!(d.max_count(), 0);
        let d = Digraph::from_bytes(&[7]);
        assert_eq!(d.max_count(), 0);
    }

    #[test]
    fn builder_spans_chunks() {
        let mut b = DigraphBuilder::new(Mode::Overlapping);
        b.push_bytes(&[0, 1]);
        b.push_bytes(&[1, 2]);
        let d = b.finish();
        assert_eq!(d.get(0, 1), 1);
        assert_eq!(d.get(1, 1), 1);
        assert_eq!(d.get(1, 2), 1);
    }

    #[test]
    fn builder_non_overlap_across_chunks() {
        let mut b = DigraphBuilder::new(Mode::NonOverlapping);
        b.push_bytes(&[1, 2, 3]);
        b.push_bytes(&[4, 5]);
        let d = b.finish();
        assert_eq!(d.get(1, 2), 1);
        assert_eq!(d.get(3, 4), 1);
        assert_eq!(d.get(2, 3), 0);
    }

    #[test]
    fn add_bytes_independent() {
        let mut d = Digraph::empty();
        d.add_bytes_with_mode(&[1, 2], Mode::Overlapping);
        d.add_bytes_with_mode(&[2, 3], Mode::Overlapping);
        assert_eq!(d.get(1, 2), 1);
        assert_eq!(d.get(2, 3), 1);
    }

    #[test]
    fn scale_clip_percentile() {
        let mut d = Digraph::empty();
        for _ in 0..100 {
            d.add_bytes_with_mode(&[5, 5], Mode::Overlapping);
        }
        d.add_bytes_with_mode(&[9, 9], Mode::Overlapping);
        let s = crate::normalize::Scale::ClipPercentile { p: 0.5 };
        let hi = s.clip_high(&d).unwrap();
        assert!(hi >= 1.0);
        let t = s.map(d.get(5, 5), d.max_count(), Some(hi));
        assert!(t <= 1.0);
    }

    #[cfg(feature = "image")]
    #[test]
    fn png_nonzero_pixel() {
        let d = Digraph::from_bytes_with_mode(&[0, 255], Mode::Overlapping);
        let img = d.to_rgba(crate::render::RenderParams::default());
        assert_eq!(img.get_pixel(0, 0).0[3], 255);
        let px = img.get_pixel(0, 0);
        assert!(px.0[0] > 0 || px.0[1] > 0 || px.0[2] > 0);
    }

    #[cfg(feature = "svg")]
    #[test]
    fn svg_contains_rect() {
        let d = Digraph::from_bytes_with_mode(&[1, 2], Mode::Overlapping);
        let svg = d.to_svg_heatmap(crate::render::SvgParams::default());
        assert!(svg.contains("<svg"));
        assert!(svg.contains("rect"));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip_json() {
        let d = Digraph::from_bytes(b"hello");
        let j = serde_json::to_string(&d).unwrap();
        let d2: Digraph = serde_json::from_str(&j).unwrap();
        assert_eq!(d, d2);
    }
}
