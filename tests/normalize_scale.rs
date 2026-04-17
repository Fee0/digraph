use digraph::digraph::{Digraph, Mode};
use digraph::normalize::Scale;

#[test]
fn log1p_hits_unit_at_max() {
    let max = 42;
    assert!((Scale::Log1p.map(max, max, None) - 1.0).abs() < 1e-6);
}

#[test]
fn clip_percentile_clamps_to_one_above_hi() {
    let s = Scale::ClipPercentile { p: 0.9 };
    let t = s.map(10, 100, Some(4.0));
    assert!((t - 1.0).abs() < 1e-6);
}

#[test]
fn clip_percentile_high_uses_nonzero_cells() {
    let mut d = Digraph::empty();
    d.add_bytes_with_mode(&[1, 2, 1, 2, 1, 2, 3, 4], Mode::Overlapping);
    let s = Scale::ClipPercentile { p: 0.5 };
    let hi = s.clip_high(&d).unwrap();
    assert!(hi >= 1.0);
    let t = s.map(d.get(1, 2), d.max_count(), Some(hi));
    assert!(t > 0.0 && t <= 1.0);
}
