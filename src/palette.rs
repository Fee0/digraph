//! Simple smooth heatmap gradients (matplotlib "magma"-like).

#[inline]
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// RGBA in linear-ish space for compositing; `t` is 0=black through 1=hot.
pub fn magma(t: f32) -> [u8; 4] {
    let t = t.clamp(0.0, 1.0);
    // Piecewise linear approximation of matplotlib magma.
    let (r, g, b) = if t < 0.25 {
        let u = t / 0.25;
        (lerp(0.0, 28.0, u), lerp(0.0, 16.0, u), lerp(4.0, 68.0, u))
    } else if t < 0.5 {
        let u = (t - 0.25) / 0.25;
        (
            lerp(28.0, 120.0, u),
            lerp(16.0, 28.0, u),
            lerp(68.0, 128.0, u),
        )
    } else if t < 0.75 {
        let u = (t - 0.5) / 0.25;
        (
            lerp(120.0, 220.0, u),
            lerp(28.0, 100.0, u),
            lerp(128.0, 220.0, u),
        )
    } else {
        let u = (t - 0.75) / 0.25;
        (
            lerp(220.0, 252.0, u),
            lerp(100.0, 254.0, u),
            lerp(220.0, 252.0, u),
        )
    };
    [r as u8, g as u8, b as u8, 255]
}

/// Viridis-like (dark purple → teal → yellow).
pub fn viridis(t: f32) -> [u8; 4] {
    let t = t.clamp(0.0, 1.0);
    let (r, g, b) = if t < 0.33 {
        let u = t / 0.33;
        (
            lerp(68.0, 33.0, u),
            lerp(1.0, 102.0, u),
            lerp(84.0, 172.0, u),
        )
    } else if t < 0.66 {
        let u = (t - 0.33) / 0.33;
        (
            lerp(33.0, 94.0, u),
            lerp(102.0, 201.0, u),
            lerp(172.0, 99.0, u),
        )
    } else {
        let u = (t - 0.66) / 0.34;
        (
            lerp(94.0, 253.0, u),
            lerp(201.0, 231.0, u),
            lerp(99.0, 37.0, u),
        )
    };
    [r as u8, g as u8, b as u8, 255]
}

/// Grayscale.
pub fn gray(t: f32) -> [u8; 4] {
    let v = (t.clamp(0.0, 1.0) * 255.0).round() as u8;
    [v, v, v, 255]
}
