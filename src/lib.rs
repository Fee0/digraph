//! Byte-pair (**digraph**) histograms for binary visualization: map each pair
//! `(b[i], b[i+1])` to a 256×256 grid (first byte = X, second = Y) and count
//! occurrences. Optional `image` / `svg` features render heatmaps.
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

#[cfg(any(feature = "image", feature = "svg"))]
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
