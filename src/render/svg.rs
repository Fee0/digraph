use crate::digraph::Digraph;
use crate::normalize::Scale;
use crate::palette::HeatmapPalette;
use std::fmt::Write as _;

/// SVG heatmap: one `<rect>` per non-zero cell to keep files smaller.
#[derive(Clone, Copy, Debug)]
pub struct SvgParams {
    /// SVG units per digraph cell (`viewBox` side is `256 * cell_size`).
    pub cell_size: f32,
    /// Count normalization strategy. Default is `Log1p` for dense outputs.
    pub scale: Scale,
    /// Palette applied after normalization.
    pub palette: HeatmapPalette,
}

impl Default for SvgParams {
    fn default() -> Self {
        Self {
            cell_size: 2.0,
            scale: Scale::Log1p,
            palette: HeatmapPalette::default(),
        }
    }
}

/// Returns an SVG document string (`viewBox` sized to the grid).
pub fn render_svg_heatmap(digraph: &Digraph, params: SvgParams) -> String {
    let cs = params.cell_size.max(0.5);
    let side = 256.0 * cs;
    let max = digraph.max_count();

    let mut s = String::new();
    let _ = writeln!(
        &mut s,
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{side}" height="{side}" viewBox="0 0 {side} {side}">"#
    );
    let _ = writeln!(
        &mut s,
        r#"<rect width="100%" height="100%" fill="rgb(11,11,18)"/>"#
    );

    // CantorDust `TwoTupleVisualizer`: rect at (second_byte, first_byte).
    for first in 0u16..256 {
        for second in 0u16..256 {
            let v = digraph.get(first as u8, second as u8);
            if v == 0 {
                continue;
            }
            let t = params.scale.map(v, max);
            let [r, g, b, a] = params.palette.rgba(t);
            let xf = second as f32 * cs;
            let yf = first as f32 * cs;
            let opacity = (a as f32) / 255.0;
            let _ = writeln!(
                &mut s,
                r#"<rect x="{xf}" y="{yf}" width="{cs}" height="{cs}" fill="rgb({r},{g},{b})" fill-opacity="{opacity}"/>"#
            );
        }
    }
    let _ = writeln!(&mut s, "</svg>");
    s
}
