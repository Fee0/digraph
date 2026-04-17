#![cfg(feature = "image")]

use digraph::{Digraph, Mode};
use digraph::render::RenderParams;

#[test]
fn png_nonzero_pixel() {
    let d = Digraph::from_bytes_with_mode(&[0, 255], Mode::Overlapping);
    let img = d.to_rgba(RenderParams::default());
    assert_eq!(img.get_pixel(0, 0).0[3], 255);
    let px = img.get_pixel(0, 0);
    assert!(px.0[0] > 0 || px.0[1] > 0 || px.0[2] > 0);
}
