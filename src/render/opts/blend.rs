use hsl::HSL;
use image::{Rgb, Rgba};

// todo: add methods to make new Blends

#[derive(Clone, Debug)]
pub enum Blend {
    /// Solid color
    None(Rgba<u8>),
    /// A mask of what RGB channels to mix together
    RgbIntensity(Rgb<bool>),
    /// A value in HSL and a min and max lightness to use.
    ///
    /// The two f64 fields must be in range (0..=1.0).
    /// The Lightness value in the HSL field will be ignored.
    HslIntensity(HSL, f64, f64),
}

impl Blend {
    pub fn solid(color: Rgba<u8>) -> Self {
        Self::None(color)
    }

    /// Blend node background colors based on an RGB mask.
    pub fn blend_rgb(mask: [bool; 3]) -> Self {
        Self::RgbIntensity(Rgb(mask))
    }

    /// Blend node background colors based on an HSL color.
    ///
    /// `min_l` and `max_l` represent the lowest and highest lightness values to use when
    /// blending intensities.
    pub fn blend_hsl(h: f64, s: f64, min_l: f64, max_l: f64) -> Result<Self, crate::Error> {
        if (h < 0.0) | (h > 360.0) {
            return Err(crate::Error::InvalidHslH(h));
        }
        if (s < 0.0) | (s > 1.0) {
            return Err(crate::Error::InvalidHslS(s));
        }
        if (min_l < 0.0) | (min_l > 1.0) {
            return Err(crate::Error::InvalidHslL(min_l));
        }
        if (max_l < 0.0) | (max_l > 1.0) {
            return Err(crate::Error::InvalidHslL(max_l));
        }

        if min_l > max_l {
            return Ok(Self::HslIntensity(hsl::HSL { h, s, l: 1.0 }, max_l, min_l));
        }

        Ok(Self::HslIntensity(hsl::HSL { h, s, l: 1.0 }, min_l, max_l))
    }
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

pub(crate) fn calc_intensity(i: f32, max: f32) -> f32 {
    (max - i) / max
}

/// Cap intensity at a specified percentage of `max`
pub(crate) fn calc_hsl_intensity(i: f64, max: f64, min_l: f64, max_l: f64) -> f64 {
    ((max - i) / max) * (max_l - min_l) + (1.0 - max_l)
}
