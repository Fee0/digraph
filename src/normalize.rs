//! Scaling counts to display intensities.

use crate::digraph::Digraph;
use core::fmt;

/// How raw counts are mapped to the unit interval before coloring.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Scale {
    /// `v / max` (or 0 if max is 0).
    Linear,
    /// `ln(1 + v) / ln(1 + max)` (stable for heavy tails).
    Log1p,
    /// Linear after clamping counts to `[0, high]` where `high` is the
    /// `p`-th percentile (0.0–1.0) of **non-zero** cells; zeros stay zero.
    ClipPercentile { p: f32 },
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Scale::Linear => f.write_str("linear"),
            Scale::Log1p => f.write_str("log1p"),
            Scale::ClipPercentile { p } => write!(f, "clip p={p:.2}"),
        }
    }
}

impl Scale {
    /// Maps a cell value `v` using global `max` and optional precomputed clip high.
    pub fn map(self, v: u32, max: u32, clip_high: Option<f32>) -> f32 {
        if v == 0 {
            return 0.0;
        }
        match self {
            Scale::Linear => {
                if max == 0 {
                    0.0
                } else {
                    (v as f32) / (max as f32)
                }
            }
            Scale::Log1p => {
                let ln1p_max = ((max as f64) + 1.0).ln_1p() as f32;
                if ln1p_max <= 0.0 {
                    0.0
                } else {
                    (v as f64).ln_1p() as f32 / ln1p_max
                }
            }
            Scale::ClipPercentile { .. } => {
                let hi = clip_high.unwrap_or(max as f32).max(1.0);
                ((v as f32) / hi).min(1.0)
            }
        }
    }

    /// Computes the high clamp for [`Scale::ClipPercentile`] from a digraph.
    pub fn clip_high(self, digraph: &Digraph) -> Option<f32> {
        match self {
            Scale::ClipPercentile { p } => Some(percentile_nonzero(digraph, p)),
            _ => None,
        }
    }
}

/// `p` in 0..=1 over sorted non-zero counts; falls back to `max` if none.
fn percentile_nonzero(digraph: &Digraph, p: f32) -> f32 {
    let p = p.clamp(0.0, 1.0);
    let mut vals: Vec<u32> = digraph
        .counts()
        .iter()
        .copied()
        .filter(|&c| c > 0)
        .collect();
    if vals.is_empty() {
        return digraph.max_count() as f32;
    }
    vals.sort_unstable();
    let idx = ((vals.len() as f32 - 1.0) * p).round() as usize;
    let idx = idx.min(vals.len() - 1);
    vals[idx] as f32
}

#[cfg(test)]
mod tests {
    use super::Scale;
    use crate::digraph::{Digraph, Mode};

    #[test]
    fn scale_clip_percentile() {
        let mut d = Digraph::empty();
        for _ in 0..100 {
            d.add_bytes_with_mode(&[5, 5], Mode::Overlapping);
        }
        d.add_bytes_with_mode(&[9, 9], Mode::Overlapping);
        let s = Scale::ClipPercentile { p: 0.5 };
        let hi = s.clip_high(&d).unwrap();
        assert!(hi >= 1.0);
        let t = s.map(d.get(5, 5), d.max_count(), Some(hi));
        assert!(t <= 1.0);
    }
}
