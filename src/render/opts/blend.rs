use image::{Rgb, Rgba};

// todo: add methods to make new Blends

#[derive(Clone, Debug)]
pub enum Blend {
    /// Solid color
    None(Rgba<u8>),
    /// A mask of what RGB channels to mix together
    RGBIntensity(Rgb<bool>),
}

pub(crate) fn intensity_color(color: &Rgb<bool>, i: f32) -> Rgba<u8> {
    let c = |col: bool| -> u8 {
        ((255f32 - col as u8 as f32 * 127.0) * i + col as u8 as f32 * 127.0) as u8
    };
    let [r, g, b] = color.0;
    Rgba([c(r), c(g), c(b), 255])
}

pub(crate) fn intensity(i: f32, max: f32) -> f32 {
    (max - i) / max
}
