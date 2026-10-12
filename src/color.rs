//! Color conversions (RGB ↔ HSL/HSV) and the AutoCAD Color Index (ACI) palette,
//! used by the colour picker (ported from XIMOD Translator).

/// Convert HSL (`h` 0–360, `s`/`l` 0–100) to `(r, g, b)`.
pub fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    let h = h.rem_euclid(360.0);
    let s = (s / 100.0).clamp(0.0, 1.0);
    let l = (l / 100.0).clamp(0.0, 1.0);
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let hp = h / 60.0;
    let x = c * (1.0 - (hp.rem_euclid(2.0) - 1.0).abs());
    let (r1, g1, b1) = match hp as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = l - c / 2.0;
    (
        ((r1 + m) * 255.0).round() as u8,
        ((g1 + m) * 255.0).round() as u8,
        ((b1 + m) * 255.0).round() as u8,
    )
}

/// Convert `(r, g, b)` to HSL (`h` 0–360, `s`/`l` 0–100).
pub fn rgb_to_hsl((r, g, b): (u8, u8, u8)) -> (f64, f64, f64) {
    let r = r as f64 / 255.0;
    let g = g as f64 / 255.0;
    let b = b as f64 / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;
    let d = max - min;
    if d.abs() < f64::EPSILON {
        return (0.0, 0.0, l * 100.0); // achromatic
    }
    let s = d / (1.0 - (2.0 * l - 1.0).abs());
    let mut h = if max == r {
        ((g - b) / d).rem_euclid(6.0)
    } else if max == g {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    } * 60.0;
    if h < 0.0 {
        h += 360.0;
    }
    (h, s * 100.0, l * 100.0)
}

/// Convert HSV (`h` 0–360, `s`/`v` 0–1) to `(r, g, b)`.
pub fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (u8, u8, u8) {
    let h = h.rem_euclid(360.0);
    let c = v * s;
    let hp = h / 60.0;
    let x = c * (1.0 - (hp.rem_euclid(2.0) - 1.0).abs());
    let (r1, g1, b1) = match hp as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = v - c;
    (
        ((r1 + m) * 255.0).round() as u8,
        ((g1 + m) * 255.0).round() as u8,
        ((b1 + m) * 255.0).round() as u8,
    )
}

/// Number of hue columns in the ACI chromatic block.
pub const ACI_HUES: usize = 24;
/// Number of shade rows per hue in the ACI chromatic block.
pub const ACI_SHADES: usize = 10;

/// RGB for an AutoCAD Color Index (1–255). Index 1–9 are the standard colors,
/// 10–249 the chromatic hue×shade grid, 250–255 a grayscale ramp. This closely
/// reproduces the ACI layout (hue across, darkening/paling down).
pub fn aci_rgb(index: u8) -> (u8, u8, u8) {
    match index {
        1 => (255, 0, 0),
        2 => (255, 255, 0),
        3 => (0, 255, 0),
        4 => (0, 255, 255),
        5 => (0, 0, 255),
        6 => (255, 0, 255),
        7 => (255, 255, 255),
        8 => (65, 65, 65),
        9 => (128, 128, 128),
        10..=249 => aci_chromatic(index),
        250..=255 => {
            let t = (index - 250) as f64 / 5.0;
            let g = (80.0 + t * (245.0 - 80.0)).round() as u8;
            (g, g, g)
        }
        _ => (0, 0, 0),
    }
}

fn aci_chromatic(index: u8) -> (u8, u8, u8) {
    let n = (index - 10) as usize;
    let hue = (n / ACI_SHADES) as f64 * (360.0 / ACI_HUES as f64);
    let k = n % ACI_SHADES;
    let lvl = k / 2;
    let (s, v) = if k.is_multiple_of(2) {
        // Fully saturated shades, value ramping bright → dark.
        (1.0, [1.0, 0.78, 0.58, 0.40, 0.24][lvl])
    } else {
        // Paler shades (toward white), also darkening.
        ([0.30, 0.38, 0.45, 0.50, 0.55][lvl], [1.0, 0.86, 0.66, 0.46, 0.28][lvl])
    };
    hsv_to_rgb(hue, s, v)
}

/// The ACI index whose color is closest to `rgb` (search 1–255). Used to show a
/// sensible index when a true-color value is chosen.
pub fn nearest_aci(rgb: (u8, u8, u8)) -> u8 {
    let (r, g, b) = (rgb.0 as i32, rgb.1 as i32, rgb.2 as i32);
    let mut best = 7u8;
    let mut best_d = i32::MAX;
    for i in 1..=255u8 {
        let (cr, cg, cb) = aci_rgb(i);
        let d = (cr as i32 - r).pow(2) + (cg as i32 - g).pow(2) + (cb as i32 - b).pow(2);
        if d < best_d {
            best_d = d;
            best = i;
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hsl_roundtrip_is_stable() {
        for &rgb in &[
            (255, 127, 255),
            (183, 223, 180),
            (10, 20, 30),
            (0, 0, 0),
            (255, 255, 255),
        ] {
            let (h, s, l) = rgb_to_hsl(rgb);
            let back = hsl_to_rgb(h, s, l);
            let close = |a: u8, b: u8| (a as i32 - b as i32).abs() <= 1;
            assert!(
                close(back.0, rgb.0) && close(back.1, rgb.1) && close(back.2, rgb.2),
                "{rgb:?} -> ({h:.0},{s:.0},{l:.0}) -> {back:?}"
            );
        }
    }

    #[test]
    fn autocad_example_value() {
        // The screenshot: H=300 S=100 L=75 → RGB 255,127,255.
        let rgb = hsl_to_rgb(300.0, 100.0, 75.0);
        assert_eq!(rgb, (255, 128, 255)); // rounding: 127.5 → 128
    }

    #[test]
    fn aci_standard_colors() {
        assert_eq!(aci_rgb(1), (255, 0, 0));
        assert_eq!(aci_rgb(5), (0, 0, 255));
        // Chromatic + grayscale ranges produce something.
        let _ = aci_rgb(211);
        assert_eq!(aci_rgb(255), (245, 245, 245));
    }

    #[test]
    fn nearest_aci_of_pure_red() {
        assert_eq!(nearest_aci((255, 0, 0)), 1);
    }
}
