use crate::opts::blend::Blend;
use image::Rgba;

// todo: add builders for Path and Arrow

#[derive(Clone, Debug)]
pub struct Path {
    /// If true override the text label to use the path step number instead of cell id
    pub label_steps: bool,
    pub arrows: Option<Rgba<u8>>,
    pub style: Arrow,
    pub path_bg: Blend,
}

impl Path {
    pub fn label_steps(&self) -> bool {
        self.label_steps
    }
    pub fn arrows(&self) -> Option<Rgba<u8>> {
        self.arrows
    }
    pub fn path_bg(&self) -> &Blend {
        &self.path_bg
    }
    pub fn style(&self) -> &Arrow {
        &self.style
    }
}

impl Default for Path {
    fn default() -> Self {
        Self {
            label_steps: true,
            // arrows: Some(Rgba([247, 64, 171, 255])),
            arrows: Some(Rgba([255, 200, 33, 255])),
            style: Arrow::default(),
            // path_bg: Blend::RGBIntensity(image::Rgb([true, false, true])),
            path_bg: Blend::HSLIntensity(
                hsl::HSL {
                    h: 195.0,
                    s: 1.0,
                    l: 0.5,
                },
                0.3,
                0.7,
            ),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Arrow {
    Straight,
    StraightCenter,
    /// Two control points from [0..1] (inclusive)
    BezierQuad(f64, f64),
}

impl Default for Arrow {
    fn default() -> Self {
        Self::StraightCenter
    }
}
