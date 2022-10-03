use hsl::HSL;
use image::{Rgb, Rgba};

// todo: add methods to make new Blends

#[derive(Clone, Debug)]
pub enum Blend {
    /// Solid color
    None(Rgba<u8>),
    /// A mask of what RGB channels to mix together
    RGBIntensity(Rgb<bool>),
    /// A value in HSL and a min and max lightness to use - must be in range (0..=1.0)
    HSLIntensity(HSL, f64, f64),
}

pub(crate) fn rgb_intensity(color: &Rgb<bool>, i: f32) -> Rgba<u8> {
    let c = |col: bool| -> u8 {
        ((255f32 - col as u8 as f32 * 127.0) * i + col as u8 as f32 * 127.0) as u8
    };
    let [r, g, b] = color.0;
    Rgba([c(r), c(g), c(b), 255])
}

pub(crate) fn hsl_intensity(color: &HSL, i: f64) -> Rgba<u8> {
    let (r, g, b) = HSL {
        h: color.h,
        s: color.s,
        l: i,
    }
    .to_rgb();
    Rgba([r, g, b, 255])
}

pub(crate) fn intensity(i: f32, max: f32) -> f32 {
    (max - i) / max
}

/// Cap intensity at a specified percentage of `max`
pub(crate) fn calc_hsl_intensity(i: f64, max: f64, min_l: f64, max_l: f64) -> f64 {
    ((max - i) / max) * (max_l - min_l) + (1.0 - max_l)
}
