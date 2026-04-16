use crate::digraph::Digraph;
use crate::normalize::Scale;
use crate::palette;
use std::fmt::Write as _;

/// SVG heatmap: one `<rect>` per non-zero cell to keep files smaller.
#[derive(Clone, Copy, Debug)]
pub struct SvgParams {
    pub cell_size: f32,
    pub scale: Scale,
    pub viridis: bool,
}

impl Default for SvgParams {
    fn default() -> Self {
        Self {
            cell_size: 2.0,
            scale: Scale::Log1p,
            viridis: false,
        }
    }
}

/// Returns an SVG document string (`viewBox` sized to the grid).
pub fn render_svg_heatmap(digraph: &Digraph, params: SvgParams) -> String {
    let cs = params.cell_size.max(0.5);
    let side = 256.0 * cs;
    let max = digraph.max_count();
    let clip = params.scale.clip_high(digraph);

    let mut s = String::new();
    let _ = writeln!(
        &mut s,
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{side}" height="{side}" viewBox="0 0 {side} {side}">"#
    );
    let _ = writeln!(
        &mut s,
        r#"<rect width="100%" height="100%" fill="rgb(11,11,18)"/>"#
    );

    for y in 0u16..256 {
        for x in 0u16..256 {
            let v = digraph.get(x as u8, y as u8);
            if v == 0 {
                continue;
            }
            let t = params.scale.map(v, max, clip);
            let [r, g, b, a] = if params.viridis {
                palette::viridis(t)
            } else {
                palette::magma(t)
            };
            let xf = x as f32 * cs;
            let yf = y as f32 * cs;
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
