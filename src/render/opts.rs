pub(crate) mod blend;
mod dist;
mod path;

pub(crate) use dist::DistOpts;
pub(crate) use path::Arrow;
pub(crate) use path::PathOpts;

use image::Rgba;

#[derive(Clone, Debug)]
pub struct GraphOpts {
    size: Size,
    colors: Colors,
    text: Text,
}

impl GraphOpts {
    pub fn new() -> GraphOptsBuilder {
        GraphOptsBuilder::new()
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn colors(&self) -> &Colors {
        &self.colors
    }

    pub fn text(&self) -> &Text {
        &self.text
    }
}

impl Default for GraphOpts {
    fn default() -> Self {
        Self {
            size: Size::default(),
            colors: Colors::default(),
            text: Text::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Size {
    block_height: u32,
    block_width: u32,
    padding: u32,
    dash_width: u32,
}

impl Size {
    pub fn new() -> SizeBuilder {
        SizeBuilder::new()
    }

    pub fn block_width(&self) -> u32 {
        self.block_width
    }

    pub fn block_height(&self) -> u32 {
        self.block_height
    }

    pub fn padding(&self) -> u32 {
        self.padding
    }

    pub fn dash_width(&self) -> u32 {
        self.dash_width
    }
}

impl Default for Size {
    fn default() -> Self {
        Self {
            block_height: 50,
            block_width: 50,
            padding: 20,
            dash_width: 3,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Colors {
    image_bg: Option<Rgba<u8>>,
    maze_bg: Option<Rgba<u8>>,
    cell_bg: Rgba<u8>,
    edges: Rgba<u8>,
    /// Dashed edges will be blended using the color's alpha channel
    dashed_edges: Rgba<u8>,
    outer_edges: Rgba<u8>,
    text: Rgba<u8>,
}

impl Colors {
    pub fn new() -> ColorsBuilder {
        ColorsBuilder::new()
    }

    pub fn image_bg(&self) -> Rgba<u8> {
        self.image_bg.unwrap_or(Rgba([255, 255, 255, 255]))
    }

    pub fn maze_bg(&self) -> Option<Rgba<u8>> {
        self.maze_bg
    }

    pub fn cell_bg(&self) -> &Rgba<u8> {
        &self.cell_bg
    }

    pub fn edges(&self) -> &Rgba<u8> {
        &self.edges
    }

    pub fn dashed_edges(&self) -> &Rgba<u8> {
        &self.dashed_edges
    }

    pub fn outer_edges(&self) -> &Rgba<u8> {
        &self.outer_edges
    }

    pub fn text(&self) -> &Rgba<u8> {
        &self.text
    }
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            image_bg: None,
            maze_bg: None,
            cell_bg: Rgba([235, 235, 235, 255]),
            edges: Rgba([80, 80, 80, 255]),
            dashed_edges: Rgba([210, 210, 210, 128]),
            outer_edges: Rgba([0, 0, 0, 255]),
            text: Rgba([0, 0, 0, 255]),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Text {
    show: bool,
    center: bool,
    width: f32,
    height: f32,
    padding: freehand::Pt<i32>,
}
impl Text {
    pub fn new() -> TextBuilder {
        TextBuilder::new()
    }

    pub fn show(&self) -> bool {
        self.show
    }

    pub fn center(&self) -> bool {
        self.center
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn padding(&self) -> freehand::Pt<i32> {
        self.padding
    }

    pub fn scale(&self) -> rusttype::Scale {
        rusttype::Scale {
            x: self.width,
            y: self.height,
        }
    }
}

impl Default for Text {
    fn default() -> Self {
        Self {
            show: true,
            center: true,
            width: 15.2f32,
            height: 15.2f32,
            padding: freehand::Pt::new(0, 0),
        }
    }
}

#[derive(Clone, Debug)]
pub struct GraphOptsBuilder {
    size: Option<Size>,
    colors: Option<Colors>,
    text: Option<Text>,
}

impl GraphOptsBuilder {
    pub fn new() -> Self {
        Self {
            size: None,
            colors: None,
            text: None,
        }
    }

    pub fn build(self) -> GraphOpts {
        GraphOpts {
            size: self.size.unwrap_or(Size::default()),
            colors: self.colors.unwrap_or(Colors::default()),
            text: self.text.unwrap_or(Text::default()),
        }
    }

    pub fn size(self, size: Size) -> Self {
        Self {
            size: Some(size),
            ..self
        }
    }

    pub fn colors(self, colors: Colors) -> Self {
        Self {
            colors: Some(colors),
            ..self
        }
    }

    pub fn text(self, text: Text) -> Self {
        Self {
            text: Some(text),
            ..self
        }
    }
}

#[derive(Clone, Debug)]
pub struct SizeBuilder {
    block_height: Option<u32>,
    block_width: Option<u32>,
    padding: Option<u32>,
    dash_width: Option<u32>,
}
impl SizeBuilder {
    pub fn new() -> Self {
        Self {
            block_height: None,
            block_width: None,
            padding: None,
            dash_width: None,
        }
    }

    pub fn build(self) -> Size {
        let defaults = Size::default();
        Size {
            block_height: self.block_height.unwrap_or(defaults.block_height),
            block_width: self.block_width.unwrap_or(defaults.block_width),
            padding: self.padding.unwrap_or(defaults.padding),
            dash_width: self.dash_width.unwrap_or(defaults.dash_width),
        }
    }

    pub fn height(self, height: u32) -> Self {
        Self {
            block_height: Some(height),
            ..self
        }
    }

    pub fn width(self, width: u32) -> Self {
        Self {
            block_width: Some(width),
            ..self
        }
    }

    pub fn padding(self, padding: u32) -> Self {
        Self {
            padding: Some(padding),
            ..self
        }
    }

    pub fn dash_width(self, dash_width: u32) -> Self {
        Self {
            dash_width: Some(dash_width),
            ..self
        }
    }

    pub fn no_dash(self) -> Self {
        Self {
            dash_width: Some(0),
            ..self
        }
    }
}

#[derive(Clone, Debug)]
pub struct ColorsBuilder {
    image_bg: Option<Rgba<u8>>,
    maze_bg: Option<Rgba<u8>>,
    cell_bg: Option<Rgba<u8>>,
    edges: Option<Rgba<u8>>,
    dashed_edges: Option<Rgba<u8>>,
    outer_edges: Option<Rgba<u8>>,
    text: Option<Rgba<u8>>,
}

impl ColorsBuilder {
    pub fn new() -> Self {
        Self {
            image_bg: None,
            maze_bg: None,
            cell_bg: None,
            edges: None,
            dashed_edges: None,
            outer_edges: None,
            text: None,
        }
    }

    pub fn build(self) -> Colors {
        let defaults = Colors::default();
        Colors {
            image_bg: self.image_bg,
            maze_bg: self.maze_bg,
            cell_bg: self.cell_bg.unwrap_or(defaults.cell_bg),
            edges: self.edges.unwrap_or(defaults.edges),
            dashed_edges: self.dashed_edges.unwrap_or(defaults.dashed_edges),
            outer_edges: self.outer_edges.unwrap_or(defaults.outer_edges),
            text: self.text.unwrap_or(defaults.text),
        }
    }

    pub fn image_bg(self, color: Rgba<u8>) -> Self {
        Self {
            image_bg: Some(color),
            ..self
        }
    }

    pub fn maze_bg(self, color: Rgba<u8>) -> Self {
        Self {
            maze_bg: Some(color),
            ..self
        }
    }

    pub fn cell_bg(self, color: Rgba<u8>) -> Self {
        Self {
            cell_bg: Some(color),
            ..self
        }
    }

    pub fn inner_edges(self, color: Rgba<u8>) -> Self {
        Self {
            edges: Some(color),
            ..self
        }
    }

    pub fn dashed_edges(self, color: Rgba<u8>) -> Self {
        Self {
            dashed_edges: Some(color),
            ..self
        }
    }

    pub fn outer_edges(self, color: Rgba<u8>) -> Self {
        Self {
            outer_edges: Some(color),
            ..self
        }
    }

    pub fn text(self, color: Rgba<u8>) -> Self {
        Self {
            text: Some(color),
            ..self
        }
    }
}

#[derive(Clone, Debug)]
pub struct TextBuilder {
    show: Option<bool>,
    center: Option<bool>,
    width: Option<f32>,
    height: Option<f32>,
    padding: Option<freehand::Pt<i32>>,
}

impl TextBuilder {
    pub fn new() -> Self {
        Self {
            show: None,
            center: None,
            width: None,
            height: None,
            padding: None,
        }
    }

    pub fn build(self) -> Text {
        let defaults = Text::default();
        Text {
            show: self.show.unwrap_or(defaults.show),
            center: self.center.unwrap_or(defaults.center),
            width: self.width.unwrap_or(defaults.width),
            height: self.height.unwrap_or(defaults.height),
            padding: self.padding.unwrap_or(defaults.padding),
        }
    }

    pub fn show(self, show: bool) -> Self {
        Self {
            show: Some(show),
            ..self
        }
    }

    pub fn center(self, center: bool) -> Self {
        Self {
            center: Some(center),
            ..self
        }
    }

    pub fn width(self, width: f32) -> Self {
        Self {
            width: Some(width),
            ..self
        }
    }

    pub fn height(self, height: f32) -> Self {
        Self {
            height: Some(height),
            ..self
        }
    }

    pub fn padding(self, x: i32, y: i32) -> Self {
        let padding = if (x >= 0) & (y >= 0) {
            freehand::Pt::new(x, y)
        } else {
            freehand::Pt::new(0, 0)
        };
        Self {
            padding: Some(padding),
            ..self
        }
    }
}
