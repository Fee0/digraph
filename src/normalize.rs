//! Scaling counts to display intensities.

use crate::digraph::Digraph;
use core::fmt;

/// How raw counts are mapped to the unit interval before coloring.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Scale {
    /// `ln(1 + v) / ln(1 + max)` (stable for heavy tails).
    Log1p,
    /// CantorDust-style fixed ramp: `min + min(v * step, 255 - min)`, then `/ 255`.
    ///
    /// This mirrors the 2-tuple visualizer's channel ramp (`min = 10`, `step = 5`)
    /// while keeping zero cells black.
    CantorDust,
    /// Linear after clamping counts to `[0, high]` where `high` is the
    /// `p`-th percentile (0.0–1.0) of **non-zero** cells; zeros stay zero.
    ClipPercentile { p: f32 },
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Scale::Log1p => f.write_str("log1p"),
            Scale::CantorDust => f.write_str("cantordust"),
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
            Scale::Log1p => {
                let ln1p_max = (max as f64).ln_1p() as f32;
                if ln1p_max <= 0.0 {
                    0.0
                } else {
                    (v as f64).ln_1p() as f32 / ln1p_max
                }
            }
            Scale::CantorDust => {
                let min = 10u32;
                let step = 5u32;
                let delta = v.saturating_mul(step).min(255 - min);
                let channel = min + delta;
                (channel as f32) / 255.0
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

    #[test]
    fn scale_cantordust_matches_fixed_ramp() {
        let s = Scale::CantorDust;
        assert_eq!(s.map(0, 10, None), 0.0);
        assert!((s.map(1, 10, None) - (15.0 / 255.0)).abs() < 1e-6);
        assert!((s.map(10, 10, None) - (60.0 / 255.0)).abs() < 1e-6);
        assert!((s.map(49, 10, None) - 1.0).abs() < 1e-6);
        assert!((s.map(10_000, 10, None) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn scale_log1p_uses_ln1p_max_denominator() {
        let s = Scale::Log1p;
        let max = 9;
        let v = 3;
        let expected = (v as f32 + 1.0).ln() / (max as f32 + 1.0).ln();
        let actual = s.map(v, max, None);
        assert!((actual - expected).abs() < 1e-6);
    }

    #[test]
    fn scale_log1p_is_monotonic_and_capped() {
        let s = Scale::Log1p;
        let max = 100;
        let a = s.map(1, max, None);
        let b = s.map(10, max, None);
        let c = s.map(100, max, None);
        assert!(a > 0.0);
        assert!(a < b && b < c);
        assert!((c - 1.0).abs() < 1e-6);
    }
}
