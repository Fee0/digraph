#![cfg(feature = "svg")]

use digraph::{Digraph, Mode};
use digraph::render::SvgParams;

#[test]
fn svg_contains_rect() {
    let d = Digraph::from_bytes_with_mode(&[1, 2], Mode::Overlapping);
    let svg = d.to_svg_heatmap(SvgParams::default());
    assert!(svg.contains("<svg"));
    assert!(svg.contains("rect"));
}
