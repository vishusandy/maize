pub mod path;
use image::Rgba;

#[derive(Clone, Debug)]
pub struct Basic {
    size: Size,
    colors: Colors,
    text: Text,
}

impl Basic {
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
impl Default for Basic {
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
    maze_bg: Option<Rgba<u8>>,
    cell_bg: Rgba<u8>,
    edges: Rgba<u8>,
    dashed_edges: Rgba<u8>,
    outer_edges: Rgba<u8>,
    text: Rgba<u8>,
}
impl Colors {
    pub fn maze_bg(&self) -> Rgba<u8> {
        self.maze_bg.unwrap_or(Rgba([255, 255, 255, 255]))
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
            // maze_bg: Rgba([180, 180, 180, 255]),
            maze_bg: None,
            cell_bg: Rgba([235, 235, 235, 255]),
            edges: Rgba([160, 160, 160, 255]),
            dashed_edges: Rgba([180, 180, 180, 255]),
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

// impl<'b> From<Basic> for std::borrow::Cow<'b, Basic> {
//     fn from(basic: Basic) -> Self {
//         std::borrow::Cow::Owned(basic)
//     }
// }

pub enum Color {
    Rgba(image::Rgba<u8>),
    Intensity(),
    BlendSolid(),
    BlendAlpha(),
}
mod color {
    pub trait Color {}
    pub struct Solid {}
    pub struct Intensity {}
    pub struct BlendSolid {
        bg: image::Rgba<u8>,
        blend: image::Rgba<u8>,
        opacity: f32,
    }
    pub struct BlendAlpha {
        blend: image::Rgba<u8>,
        opacity: f32,
    }
}
