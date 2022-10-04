use crate::opts::blend::Blend;

#[derive(Clone, Debug)]
pub struct DistOpts {
    label_dist: bool,
    bg: Blend,
}

impl DistOpts {
    pub fn label_dist(&self) -> bool {
        self.label_dist
    }

    pub fn bg(&self) -> &Blend {
        &self.bg
    }
}

impl Default for DistOpts {
    fn default() -> Self {
        Self {
            label_dist: true,
            bg: Blend::RgbIntensity(image::Rgb([true, false, true])),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DistOptsBuilder {
    label_dist: Option<bool>,
    bg: Option<Blend>,
}

impl DistOptsBuilder {
    pub fn new() -> Self {
        Self {
            label_dist: None,
            bg: None,
        }
    }

    pub fn build(self) -> DistOpts {
        let defaults = DistOpts::default();
        DistOpts {
            label_dist: self.label_dist.unwrap_or(defaults.label_dist),
            bg: self.bg.unwrap_or(defaults.bg),
        }
    }

    pub fn show_dist(self, show_dist: bool) -> Self {
        Self {
            label_dist: Some(show_dist),
            ..self
        }
    }

    pub fn path_bg(self, bg: Blend) -> Self {
        Self {
            bg: Some(bg),
            ..self
        }
    }
}
