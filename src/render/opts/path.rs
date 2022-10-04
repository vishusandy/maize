use crate::opts::blend::Blend;
use image::Rgba;

#[derive(Clone, Debug)]
pub struct PathOpts {
    /// If true override the text label to use the path step number instead of cell id
    label_steps: bool,
    arrows: Option<Rgba<u8>>,
    style: Arrow,
    path_bg: Blend,
}

impl PathOpts {
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

impl Default for PathOpts {
    fn default() -> Self {
        Self {
            label_steps: true,
            arrows: Some(Rgba([255, 200, 33, 255])),
            style: Arrow::default(),
            path_bg: Blend::blend_rgb([true, false, true]),
            // path_bg: Blend::blend_hsl(195.0, 1.0, 0.3, 0.7),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Arrow {
    /// Draw a straight line directly from one edge to the other
    Straight,
    /// Draw a straight line from each edge to the center.
    StraightCenter,
    /// Two control points from 0  to 1 (inclusive).
    ///
    /// For more see: [Bézier curve - Wikipedia](https://en.wikipedia.org/wiki/B%C3%A9zier_curve)
    BezierQuad(f64, f64),
}

impl Default for Arrow {
    fn default() -> Self {
        Self::StraightCenter
    }
}

#[derive(Clone, Debug)]
pub struct PathOptsBuilder {
    label_steps: Option<bool>,
    arrows: Option<Option<Rgba<u8>>>,
    style: Option<Arrow>,
    path_bg: Option<Blend>,
}

impl PathOptsBuilder {
    pub fn new() -> Self {
        Self {
            label_steps: None,
            arrows: None,
            style: None,
            path_bg: None,
        }
    }

    pub fn build(self) -> PathOpts {
        let defaults = PathOpts::default();
        PathOpts {
            label_steps: self.label_steps.unwrap_or(defaults.label_steps),
            arrows: self.arrows.unwrap_or(defaults.arrows),
            style: self.style.unwrap_or(defaults.style),
            path_bg: self.path_bg.unwrap_or(defaults.path_bg),
        }
    }

    pub fn show_steps(self, show_steps: bool) -> Self {
        Self {
            label_steps: Some(show_steps),
            ..self
        }
    }

    pub fn arrow_color(self, color: Option<Rgba<u8>>) -> Self {
        Self {
            arrows: Some(color),
            ..self
        }
    }

    pub fn arrow_style(self, style: Arrow) -> Self {
        Self {
            style: Some(style),
            ..self
        }
    }

    pub fn path_bg(self, bg: Blend) -> Self {
        Self {
            path_bg: Some(bg),
            ..self
        }
    }
}
