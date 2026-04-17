//! Print a downsampled ASCII digraph heatmap for a binary file.
//!
//! Default input is `tests/data/sample.bin`. Other format-shaped fixtures live in
//! `tests/data/` (e.g. `pe_like.bin`, `elf_like.bin`, `zip_like.bin`, `png_like.bin`,
//! `jpeg_like.bin`, `gzip_like.bin`); regenerate all via `python scripts/gen_test_binaries.py`.
//!
//! ```text
//! cargo run --example ascii_from_file
//! cargo run --example ascii_from_file -- path/to/file.bin
//! ```

use digraph::render::AsciiParams;
use digraph::{Digraph, Mode};
use std::env;
use std::path::PathBuf;

fn default_input() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/sample.bin")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: PathBuf = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(default_input);

    let bytes = std::fs::read(&input)?;
    let digraph = Digraph::from_bytes_with_mode(&bytes, Mode::Overlapping);

    eprintln!(
        "read {} bytes from {}; max cell count = {}",
        bytes.len(),
        input.display(),
        digraph.max_count()
    );

    let params = AsciiParams {
        cols: 80,
        rows: 36,
        ..AsciiParams::default()
    };
    print!("{}", digraph.to_ascii(params));
    Ok(())
}
