//! Render a byte-pair digraph heatmap from a binary file to an SVG.
//!
//! From the crate root (uses `tests/data/lore.txt` by default):
//!
//! ```text
//! cargo run --example svg_from_file --features svg
//! ```
//!
//! Custom paths and color palette (`magma`, `viridis`, `gray`, or `matrix`):
//!
//! ```text
//! cargo run --example svg_from_file --features svg -- path/to/file.bin out.svg viridis
//! ```

use digraph::SvgParams;
use digraph::{Digraph, HeatmapPalette, Mode};
use std::env;
use std::path::PathBuf;

fn default_input() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/lore.txt")
}

fn default_output() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/digraph-from-file.svg")
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

    let params = SvgParams {
        cell_size: 3.0,
        palette,
        ..SvgParams::default()
    };
    let svg = digraph.to_svg_heatmap(params);
    std::fs::write(&output, svg)?;

    eprintln!("wrote SVG to {}", output.display());
    Ok(())
}
