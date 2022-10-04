pub(crate) mod rect;

use crate::graphs::orth::Orth;
use crate::render::state::graph::{Builder, BuilderGraph};
use image::RgbaImage;

impl Orth<rect::RectCell> {
    /// Use default rendering options to render an image
    pub fn render(&self) -> RgbaImage {
        self.build_render().finish().render()
    }

    pub fn build_render<'g>(&'g self) -> BuilderGraph<'g, Self> {
        Builder::graph(self)
    }

    pub fn build_render_owned<'g>(self) -> BuilderGraph<'g, Self> {
        Builder::owned_graph(self)
    }

    pub fn dist(&self) -> crate::Dist {
        crate::Dist::blank(self)
    }

    pub fn shortest_path(
        &self,
        dist: &crate::Dist,
        end: usize,
    ) -> Result<crate::Path, crate::Error> {
        dist.shortest_path(self, end)
    }

    fn above(id: usize, width: usize) -> Option<usize> {
        if id < width {
            None
        } else {
            Some(id - width)
        }
    }

    fn below(id: usize, width: usize, len: usize) -> Option<usize> {
        if id >= len - width {
            None
        } else {
            Some(id + width)
        }
    }

    fn left(id: usize, width: usize) -> Option<usize> {
        if id % width == 0 {
            None
        } else {
            Some(id - 1)
        }
    }

    fn right(id: usize, width: usize) -> Option<usize> {
        if id % width == width - 1 {
            None
        } else {
            Some(id + 1)
        }
    }

    fn new_cell(id: usize, width: usize, len: usize) -> rect::RectCell {
        rect::RectCell {
            id,
            row: id as u32 / width as u32,
            col: id as u32 % width as u32,
            n: [
                Self::above(id, width),
                Self::right(id, width),
                Self::below(id, width, len),
                Self::left(id, width),
            ],
            links: [None; 4],
        }
    }

    fn new_cell_linked(id: usize, width: usize, len: usize) -> rect::RectCell {
        rect::RectCell {
            id,
            row: id as u32 / width as u32,
            col: id as u32 % width as u32,
            n: [
                Self::above(id, width),
                Self::right(id, width),
                Self::below(id, width, len),
                Self::left(id, width),
            ],
            links: [
                Self::above(id, width),
                Self::right(id, width),
                Self::below(id, width, len),
                Self::left(id, width),
            ],
        }
    }

    pub fn new(height: usize, width: usize) -> Self {
        let len = height * width;
        Self {
            len,
            height,
            width,
            cells: (0..len).map(|id| Self::new_cell(id, width, len)).collect(),
        }
    }

    pub fn new_linked(height: usize, width: usize) -> Self {
        let len = height * width;
        Self {
            len,
            height,
            width,
            cells: (0..len)
                .map(|id| Self::new_cell_linked(id, width, len))
                .collect(),
        }
    }
}
