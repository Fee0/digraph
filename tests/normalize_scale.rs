use digraph::normalize::Scale;

#[test]
fn log1p_hits_unit_at_max() {
    let max = 42;
    assert!((Scale::Log1p.map(max, max) - 1.0).abs() < 1e-6);
}
