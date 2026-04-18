#![cfg(feature = "image")]

use digraph::{Digraph, Mode};
use digraph::RenderParams;

#[test]
fn png_places_pair_at_expected_coordinates() {
    let d = Digraph::from_bytes_with_mode(&[1, 2], Mode::Overlapping);
    let img = d.to_rgba(RenderParams::default());

    // CantorDust layout: x = second byte, y = first byte.
    let target = img.get_pixel(2 * 2, 1 * 2);
    assert_eq!(target.0[3], 255);
    assert!(target.0[0] > 0 || target.0[1] > 0 || target.0[2] > 0);

    let empty = img.get_pixel(0, 0);
    assert_eq!(empty.0[3], 255);
    assert_ne!(target.0, empty.0, "active cell should differ from background");
}

#[test]
fn png_non_overlapping_ignores_tail_byte() {
    let d = Digraph::from_bytes_with_mode(&[9, 10, 11], Mode::NonOverlapping);
    let img = d.to_rgba(RenderParams::default());
    let px = img.get_pixel(10 * 2, 9 * 2);
    assert!(px.0[0] > 0 || px.0[1] > 0 || px.0[2] > 0);
}
