#![cfg(feature = "serde")]

use digraph::Digraph;

#[test]
fn serde_roundtrip_json() {
    let d = Digraph::from_bytes(b"hello");
    let j = serde_json::to_string(&d).unwrap();
    let d2: Digraph = serde_json::from_str(&j).unwrap();
    assert_eq!(d, d2);
}
