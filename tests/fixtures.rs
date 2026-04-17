use digraph::Digraph;
use std::path::PathBuf;

#[test]
fn fixture_sample_bin_yields_pairs() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/sample.bin");
    let bytes = std::fs::read(&path).expect("tests/data/sample.bin missing");
    let d = Digraph::from_bytes(&bytes);
    assert!(d.max_count() > 0, "fixture should contain repeated bigrams");
}

#[test]
fn format_fixtures_have_magic_and_bigrams() {
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
