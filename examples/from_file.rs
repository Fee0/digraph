//! Render a byte-pair digraph heatmap from a binary file to a PNG.
//!
//! From the crate root (uses the bundled `tests/data/sample.bin` by default):
//!
//! ```text
//! cargo run --example from_file --features image
//! ```
//!
//! Custom paths:
//!
//! ```text
//! cargo run --example from_file --features image -- path/to/file.bin out.png
//! ```

use digraph::render::RenderParams;
use digraph::{Digraph, Mode};
use std::env;
use std::fs::File;
use std::path::PathBuf;

fn default_input() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/sample.bin")
}

fn default_output() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/digraph-from-file.png")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let input: PathBuf = args.next().map(PathBuf::from).unwrap_or_else(default_input);
    let output: PathBuf = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(default_output);

    let bytes = std::fs::read(&input)?;
    let digraph = Digraph::from_bytes_with_mode(&bytes, Mode::Overlapping);

    eprintln!(
        "read {} bytes from {}; max cell count = {}",
        bytes.len(),
        input.display(),
        digraph.max_count()
    );

    if let Some(dir) = output.parent() {
        std::fs::create_dir_all(dir)?;
    }

    let file = File::create(&output)?;
    let params = RenderParams {
        cell_pixels: 3,
        ..RenderParams::default()
    };
    digraph.to_png(params, file)?;

    eprintln!("wrote PNG to {}", output.display());
    Ok(())
}
