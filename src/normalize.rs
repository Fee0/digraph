//! Scaling counts to display intensities.

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
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Scale::Log1p => f.write_str("log1p"),
            Scale::CantorDust => f.write_str("cantordust"),
        }
    }
}

impl Scale {
    /// Maps a cell value `v` using global `max`.
    pub fn map(self, v: u32, max: u32) -> f32 {
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Scale;

    #[test]
    fn scale_cantordust_matches_fixed_ramp() {
        let s = Scale::CantorDust;
        assert_eq!(s.map(0, 10), 0.0);
        assert!((s.map(1, 10) - (15.0 / 255.0)).abs() < 1e-6);
        assert!((s.map(10, 10) - (60.0 / 255.0)).abs() < 1e-6);
        assert!((s.map(49, 10) - 1.0).abs() < 1e-6);
        assert!((s.map(10_000, 10) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn scale_log1p_uses_ln1p_max_denominator() {
        let s = Scale::Log1p;
        let max = 9;
        let v = 3;
        let expected = (v as f32 + 1.0).ln() / (max as f32 + 1.0).ln();
        let actual = s.map(v, max);
        assert!((actual - expected).abs() < 1e-6);
    }

    #[test]
    fn scale_log1p_is_monotonic_and_capped() {
        let s = Scale::Log1p;
        let max = 100;
        let a = s.map(1, max);
        let b = s.map(10, max);
        let c = s.map(100, max);
        assert!(a > 0.0);
        assert!(a < b && b < c);
        assert!((c - 1.0).abs() < 1e-6);
    }
}
