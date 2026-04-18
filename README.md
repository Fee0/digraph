# digraph

Byte-pair (**digraph**) histograms for binary visualization: turn a byte stream into a 256×256 frequency map so structure shows up as patterns in a heatmap.

You pass bytes (from a file, network, or memory) and use one of different renderers to render a visualization.

Useful to reverse engineer unknown byte streams.

![Digraph heatmap: 256×256 grid of consecutive byte-pair frequencies](assets/digraph-from-file.png)

## Why digraphs?

Sequential byte pairs `(b[i], b[i+1])` form a compact “fingerprint” of raw data: repeated regions, alignment, text-like ranges, and mixed blobs often produce recognizable shapes when pair counts are shown as an image.

## What this library does

- Builds a 256×256 table: cell `(x, y)` counts how often byte `x` is immediately followed by byte `y`.
- **Layout for heatmaps**: row index = first byte of the pair, column index = second byte (CantorDust-style two-tuple orientation used in many binary-viz tools).
- **Modes** (`Mode`): overlapping pairs `(0,1), (1,2), …` or non-overlapping `(0,1), (2,3), …`.
- **Rendering**: terminal ASCII heatmap, raw RGBA pixmap, PNG (`image` feature) and SVG (`svg` feature).
- **Optional** `serde` for serializing `Digraph` / `Mode`.

## Example

Read a file, build the digraph, write a PNG (`image` feature required for `to_png`):

```toml
[dependencies]
digraph = { version = "0.1", features = ["image"] }
```

```rust
use digraph::{Digraph, Mode, RenderParams};
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = std::fs::read("input.bin")?;
    let d = Digraph::from_bytes_with_mode(&bytes, Mode::Overlapping);
    d.to_png(RenderParams::default(), File::create("digraph.png")?)?;
    Ok(())
}
```

## Features

| Feature   | Enables |
|-----------|---------|
| *(none)* | Core digraph, ASCII, raw RGBA pixmap |
| `image`  | PNG helpers, `RgbaImage` via `image` |
| `svg`    | SVG heatmap strings |
| `serde`  | `Serialize` / `Deserialize` on core types |

## Further reading

- [Battelle publishes open-source binary visualization tool](https://inside.battelle.org/blog-details/battelle-publishes-open-source-binary-visualization-tool) 