#![cfg(feature = "svg")]

use digraph::{Digraph, Mode};
use digraph::SvgParams;

#[test]
fn svg_contains_exact_rect_coordinates() {
    let d = Digraph::from_bytes_with_mode(&[1, 2], Mode::Overlapping);
    let svg = d.to_svg_heatmap(SvgParams { cell_size: 2.0, ..SvgParams::default() });
    assert!(svg.contains("<svg"));
    assert!(svg.contains(r#"x="4" y="2" width="2" height="2""#));
}

#[test]
fn svg_omits_zero_cells() {
    let d = Digraph::empty();
    let svg = d.to_svg_heatmap(SvgParams::default());
    // The background rect is always present; no data rects should be emitted.
    let rect_count = svg.matches("<rect ").count();
    assert_eq!(rect_count, 1);
}
