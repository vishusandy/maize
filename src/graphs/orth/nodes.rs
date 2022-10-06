pub(crate) mod rect;

use crate::graphs::orth::Orth;
use crate::render::state::graph::{Builder, BuilderGraph};
use image::RgbaImage;

impl Orth<rect::RectCell> {
    /// Create a new rectangular grid without any links between cells
    pub fn new(height: usize, width: usize) -> Self {
        let len = height * width;
        Self {
            len,
            height,
            width,
            cells: (0..len).map(|id| Self::new_cell(id, width, len)).collect(),
        }
    }

    /// Create a new rectangular grid with all possible cells linked
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

    /// Use default rendering options to render an image
    pub fn render(&self) -> RgbaImage {
        self.build_render().finish().render()
    }

    /// Customize the renderer using a reference to the current graph (preferred)
    pub fn build_render(&self) -> BuilderGraph<'_, Self> {
        Builder::graph(self)
    }

    /// Customize the renderer using the current owned graph
    pub fn build_render_owned<'g>(self) -> BuilderGraph<'g, Self> {
        Builder::owned_graph(self)
    }

    /// Create a distance map from the current graph and a given starting point
    pub fn dist(&self, start: usize) -> crate::Dist {
        crate::Dist::simple(self, start)
    }

    /// Find the shortest path using a given distance map
    pub fn shortest_path(
        &self,
        dist: &crate::Dist,
        end: usize,
    ) -> Result<crate::Path, crate::Error> {
        dist.shortest_path(self, end)
    }

    /// Find the shortest path using a weight of 1 for all edges
    pub fn shortest_path_simple(
        &self,
        start: usize,
        end: usize,
    ) -> Result<crate::Path, crate::Error> {
        self.dist(start).shortest_path(self, end)
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
}
