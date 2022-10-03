use crate::opts::blend::Blend;

#[derive(Clone, Debug)]
pub struct Dist {
    label_dist: bool,
    bg: Blend,
}

impl Dist {
    pub fn label_dist(&self) -> bool {
        self.label_dist
    }
    pub fn bg(&self) -> &Blend {
        &self.bg
    }
}

impl Default for Dist {
    fn default() -> Self {
        Self {
            label_dist: true,
            bg: Blend::RGBIntensity(image::Rgb([true, false, true])),
        }
    }
}
