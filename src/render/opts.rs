use image::Rgba;

#[derive(Clone, Debug)]
pub(crate) struct Basic {
    size: Size,
    colors: Colors,
    text: Text,
}

impl Basic {
    pub(crate) fn size(&self) -> &Size {
        &self.size
    }
    pub(crate) fn colors(&self) -> &Colors {
        &self.colors
    }
    pub(crate) fn text(&self) -> &Text {
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
pub(crate) struct Size {
    block_height: u32,
    block_width: u32,
    padding: u32,
}
impl Size {
    pub(crate) fn block_width(&self) -> u32 {
        self.block_width
    }
    pub(crate) fn block_height(&self) -> u32 {
        self.block_height
    }
    pub(crate) fn padding(&self) -> u32 {
        self.padding
    }
}
impl Default for Size {
    fn default() -> Self {
        Self {
            block_height: 50,
            block_width: 50,
            padding: 20,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Colors {
    maze_bg: Option<Rgba<u8>>,
    cell_bg: Rgba<u8>,
    edges: Rgba<u8>,
    outer_edges: Rgba<u8>,
    text: Rgba<u8>,
}
impl Colors {
    pub(crate) fn cell_bg(&self) -> &Rgba<u8> {
        &self.cell_bg
    }
    pub(crate) fn edges(&self) -> &Rgba<u8> {
        &self.edges
    }
    pub(crate) fn outer_edges(&self) -> &Rgba<u8> {
        &self.outer_edges
    }
}
impl Default for Colors {
    fn default() -> Self {
        Self {
            // maze_bg: Rgba([180, 180, 180, 255]),
            maze_bg: None,
            cell_bg: Rgba([220, 220, 220, 255]),
            edges: Rgba([240, 240, 240, 255]),
            outer_edges: Rgba([0, 0, 0, 255]),
            text: Rgba([0, 0, 0, 255]),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Text {
    show: bool,
    center: bool,
    width: f32,
    height: f32,
}
impl Text {
    pub(crate) fn show(&self) -> bool {
        self.show
    }
    pub(crate) fn center(&self) -> bool {
        self.center
    }
    pub(crate) fn width(&self) -> f32 {
        self.width
    }
    pub(crate) fn height(&self) -> f32 {
        self.height
    }
}
impl Default for Text {
    fn default() -> Self {
        Self {
            show: true,
            center: true,
            width: 15.2f32,
            height: 15.2f32,
        }
    }
}
