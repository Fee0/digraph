//! Render a byte-pair digraph heatmap from a binary file to a PNG.
//!
//! From the crate root (uses the bundled `tests/data/sample.bin` by default; see also
//! `tests/data/*_like.bin` fixtures from `python scripts/gen_test_binaries.py`):
//!
//! ```text
//! cargo run --example from_file --features image
//! ```
//!
//! Custom paths and color palette (`magma`, `viridis`, `gray`, or `matrix`):
//!
//! ```text
//! cargo run --example from_file --features image -- path/to/file.bin out.png viridis
//! ```

use digraph::RenderParams;
use digraph::{Digraph, HeatmapPalette, Mode};
use std::env;
use std::fs::File;
use std::path::PathBuf;

fn default_input() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/lore.txt")
}

fn default_output() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/digraph-from-file.png")
}

fn parse_palette(s: &str) -> Result<HeatmapPalette, String> {
    match s.to_ascii_lowercase().as_str() {
        "magma" | "m" => Ok(HeatmapPalette::Magma),
        "viridis" | "v" => Ok(HeatmapPalette::Viridis),
        "gray" | "grey" | "g" => Ok(HeatmapPalette::Gray),
        "matrix" | "x" => Ok(HeatmapPalette::Matrix),
        _ => Err(format!(
            "unknown palette {s:?} (expected magma, viridis, gray, or matrix)"
        )),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    let (input, output, palette) = match args.len() {
        0 => (default_input(), default_output(), HeatmapPalette::default()),
        1 => (
            PathBuf::from(&args[0]),
            default_output(),
            HeatmapPalette::default(),
        ),
        2 => (
            PathBuf::from(&args[0]),
            PathBuf::from(&args[1]),
            HeatmapPalette::default(),
        ),
        _ => (
            PathBuf::from(&args[0]),
            PathBuf::from(&args[1]),
            parse_palette(&args[2])
                .map_err(|msg| std::io::Error::new(std::io::ErrorKind::InvalidInput, msg))?,
        ),
    };

    let bytes = std::fs::read(&input)?;
    let digraph = Digraph::from_bytes_with_mode(&bytes, Mode::Overlapping);

    eprintln!(
        "read {} bytes from {}; max cell count = {}; palette = {:?}",
        bytes.len(),
        input.display(),
        digraph.max_count(),
        palette
    );

    if let Some(dir) = output.parent() {
        std::fs::create_dir_all(dir)?;
    }

    let file = File::create(&output)?;
    let params = RenderParams {
        cell_pixels: 3,
        palette,
        ..RenderParams::default()
    };
    digraph.to_png(params, file)?;

    eprintln!("wrote PNG to {}", output.display());
    Ok(())
}
