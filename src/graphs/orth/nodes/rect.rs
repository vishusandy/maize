use crate::error::Error;
use crate::graphs::orth::Orth;
use crate::graphs::{Block, Neighbors, Node};
use crate::render::opts::blend::Blend;
use freehand::*;
use image::{Rgba, RgbaImage};

#[derive(Clone, Debug)]
pub struct RectBlock {
    nw: Pt<u32>,
    ne: Pt<u32>,
    se: Pt<u32>,
    sw: Pt<u32>,
}
impl RectBlock {
    fn n(&self) -> (Pt<u32>, Pt<u32>) {
        (self.nw, self.ne)
    }
    fn e(&self) -> (Pt<u32>, Pt<u32>) {
        (self.ne, self.se)
    }
    fn s(&self) -> (Pt<u32>, Pt<u32>) {
        (self.sw, self.se)
    }
    fn w(&self) -> (Pt<u32>, Pt<u32>) {
        (self.nw, self.sw)
    }
    fn center(&self) -> Pt<u32> {
        Pt::new(
            (self.nw.x() + self.se.x()) / 2,
            (self.nw.y() + self.se.y()) / 2,
        )
    }
    fn mid_n(&self) -> Pt<u32> {
        Pt::new((self.nw.x() + self.ne.x()) / 2, self.nw.y())
    }
    fn mid_s(&self) -> Pt<u32> {
        Pt::new((self.sw.x() + self.se.x()) / 2, self.sw.y())
    }
    fn mid_w(&self) -> Pt<u32> {
        Pt::new(self.nw.x(), (self.nw.y() + self.sw.y()) / 2)
    }
    fn mid_e(&self) -> Pt<u32> {
        Pt::new(self.ne.x(), (self.ne.y() + self.se.y()) / 2)
    }
    fn mid(&self, n: usize) -> Pt<u32> {
        match n {
            0 => self.mid_n(),
            1 => self.mid_e(),
            2 => self.mid_s(),
            3 => self.mid_w(),
            _ => panic!("Invalid edge {}", n),
        }
    }
    fn draw_n(&self, image: &mut RgbaImage, color: Rgba<u8>) {
        let p = self.n();
        horizontal_line(image, p.0.y() - 1, p.0.x(), p.1.x(), color);
    }
    fn draw_s(&self, image: &mut RgbaImage, color: Rgba<u8>) {
        let p = self.s();
        horizontal_line(image, p.0.y(), p.0.x(), p.1.x(), color);
    }
    fn draw_w(&self, image: &mut RgbaImage, color: Rgba<u8>) {
        let p = self.w();
        vertical_line(image, p.0.x(), p.0.y() - 1, p.1.y(), color);
    }
    fn draw_e(&self, image: &mut RgbaImage, color: Rgba<u8>) {
        let p = self.e();
        vertical_line(image, p.0.x(), p.0.y() - 1, p.1.y(), color);
    }
    fn draw_side(&self, n: usize, image: &mut RgbaImage, color: Rgba<u8>) {
        match n {
            0 => self.draw_n(image, color),
            1 => self.draw_e(image, color),
            2 => self.draw_s(image, color),
            3 => self.draw_w(image, color),
            _ => {}
        }
    }
    fn draw_dashed_n(&self, width: u32, image: &mut RgbaImage, color: Rgba<u8>) {
        let p = self.n();
        horizontal_dashed_line(image, p.0.y() - 1, p.0.x(), p.1.x(), width, color);
    }
    fn draw_dashed_s(&self, width: u32, image: &mut RgbaImage, color: Rgba<u8>) {
        let p = self.s();
        horizontal_dashed_line(image, p.0.y(), p.0.x(), p.1.x(), width, color);
    }
    fn draw_dashed_w(&self, width: u32, image: &mut RgbaImage, color: Rgba<u8>) {
        let p = self.w();
        vertical_dashed_line(image, p.0.x(), p.0.y() - 1, p.1.y(), width, color);
    }
    fn draw_dashed_e(&self, width: u32, image: &mut RgbaImage, color: Rgba<u8>) {
        let p = self.e();
        vertical_dashed_line(image, p.0.x(), p.0.y() - 1, p.1.y(), width, color);
    }
    fn draw_dashed_side(&self, n: usize, width: u32, image: &mut RgbaImage, color: Rgba<u8>) {
        match n {
            0 => self.draw_dashed_n(width, image, color),
            1 => self.draw_dashed_e(width, image, color),
            2 => self.draw_dashed_s(width, image, color),
            3 => self.draw_dashed_w(width, image, color),
            _ => {}
        }
    }
}
impl Block for RectBlock {}

#[derive(Clone, Debug)]
pub struct RectCell {
    id: usize,
    row: u32,
    col: u32,
    n: [Option<usize>; 4], // neighbors
    links: [Option<usize>; 4],
}

impl Node for RectCell {
    type Block = RectBlock;
    const N: usize = 4;
    fn id(&self) -> usize {
        self.id
    }
    fn all_neighbors(&self) -> &[Option<usize>] {
        &self.n
    }
    fn neighbors(&self) -> crate::graphs::Neighbors {
        crate::graphs::Neighbors::new(&self.n[..])
    }
    fn num_neighbors(&self) -> usize {
        self.n
            .iter()
            .fold(0usize, |acc, n| acc + n.is_some() as usize)
    }
    fn neighbor(&self, n: usize) -> Option<usize> {
        self.n[n]
    }
    fn neighbor_id(&self, cell: usize) -> Option<usize> {
        self.n.iter().position(|n| match n {
            Some(n) if *n == cell => true,
            _ => false,
        })
    }
    fn links(&self) -> Neighbors {
        Neighbors::new(&self.links)
    }
    fn linked_to(&self, id: usize) -> bool {
        match self.neighbor_id(id) {
            Some(n) => self.links[n].is_some(),
            None => false,
        }
    }
    fn linked_side(&self, n: usize) -> bool {
        self.links[n].is_some()
    }
    fn link(&mut self, cell: usize) -> Result<(), Error> {
        match self.neighbor_id(cell) {
            None => Err(Error::InvalidNeighbor(self.id, cell)),
            Some(n) => {
                if self.links[n].is_none() {
                    self.links[n] = Some(cell);
                    Ok(())
                } else {
                    Err(Error::AlreadyLinked(self.id, cell))
                }
            }
        }
    }
    fn unlink(&mut self, cell: usize) -> Result<(), Error> {
        match self.neighbor_id(cell) {
            None => Err(Error::InvalidNeighbor(self.id, cell)),
            Some(n) => {
                if self.links[n].is_some() {
                    self.links[n] = None;
                    Ok(())
                } else {
                    Err(Error::AlreadyUnlinked(self.id, cell))
                }
            }
        }
    }
}

impl crate::render::RenderBlock for RectCell {
    fn block(&self, height: u32, width: u32, padding: u32) -> Self::Block {
        // the +1 accounts for borders, assuming border size of 1
        let x = width * (self.col) + self.col + padding;
        let y = height * (self.row) + self.row + padding;
        RectBlock {
            nw: Pt::new(x, y),
            ne: Pt::new(x + width, y),
            se: Pt::new(x + width, y + height),
            sw: Pt::new(x, y + height),
        }
    }
    fn fill(&self, block: &Self::Block, color: &Rgba<u8>, image: &mut RgbaImage) {
        let width = block.ne.x() - block.nw.x();
        let height = block.sw.y() - block.nw.y();
        rectangle_filled(image, block.nw, height, width, *color);
    }
    fn blend_fill(
        &self,
        block: &Self::Block,
        i: usize,
        max: usize,
        blend: &Blend,
        image: &mut RgbaImage,
    ) {
        use crate::render::opts::blend::{intensity, intensity_color};
        match blend {
            Blend::None(color) => self.fill(block, &color, image),
            Blend::RGBIntensity(color) => {
                let int = intensity(i as f32, max as f32);
                let col = intensity_color(color, int);
                #[cfg(test)]
                log::debug!("col={:?} i={} max={} int={:.01}", col, i, max, int);
                self.fill(block, &col, image)
            }
        }
    }
    fn edge_unlinked(
        &self,
        block: &Self::Block,
        n: usize,
        color: &Rgba<u8>,
        image: &mut RgbaImage,
    ) {
        block.draw_side(n, image, *color);
    }
    fn edge_linked(
        &self,
        block: &Self::Block,
        n: usize,
        width: u32,
        color: &Rgba<u8>,
        image: &mut RgbaImage,
    ) {
        block.draw_dashed_side(n, width, image, *color);
    }
    fn text_pos(&self, block: &Self::Block, center: bool, padding: Pt<i32>) -> Pt<u32> {
        let width = (block.ne.x() - block.nw.x()) as i32;
        let height = (block.sw.y() - block.nw.y()) as i32;

        if !center {
            Pt::new(
                block.nw.x() as i32 + padding.x(),
                block.nw.y() as i32 + padding.y(),
            )
            .u32()
        } else {
            Pt::new(
                block.nw.x() as i32 + width / 2 + padding.x(),
                block.nw.y() as i32 + height / 2 + padding.y(),
            )
            .u32()
        }
    }
    fn arrow(
        &self,
        block: &Self::Block,
        from_n: usize,
        to_n: usize,
        style: &crate::render::opts::Arrow,
        color: Rgba<u8>,
        image: &mut RgbaImage,
    ) {
        use crate::render::opts::Arrow;
        match style {
            Arrow::Straight => imageproc::drawing::draw_line_segment_mut(
                image,
                block.mid(from_n).f32().into(),
                block.mid(to_n).f32().into(),
                color,
            ),
            Arrow::StraightCenter => {
                self.half_arrow(block, from_n, style, color, image);
                self.half_arrow(block, to_n, style, color, image);
            }
            // todo: bezier curves
            _ => todo!(),
        }
    }
    fn half_arrow(
        &self,
        block: &Self::Block,
        n: usize,
        style: &crate::render::opts::Arrow,
        color: Rgba<u8>,
        image: &mut RgbaImage,
    ) {
        use crate::render::opts::Arrow;
        match style {
            Arrow::Straight | Arrow::StraightCenter => imageproc::drawing::draw_line_segment_mut(
                image,
                block.mid(n).f32().into(),
                block.center().f32().into(),
                color,
            ),
            // todo: bezier curves
            _ => todo!(),
        }
    }
}

use crate::render::state::graph::{Builder, BuilderGraph};
impl Orth<RectCell> {
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

    fn new_cell(id: usize, width: usize, len: usize) -> RectCell {
        RectCell {
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

    fn new_cell_linked(id: usize, width: usize, len: usize) -> RectCell {
        RectCell {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_rect_grid() {
        crate::logger(log::LevelFilter::Warn);
        use crate::graphs::Graph;
        let grid = Orth::new(3, 3);
        log::debug!("{:#?}", grid);
        for (i, cell) in grid.cells().enumerate() {
            assert_eq!(i, cell.id);
        }
        assert_eq!(grid.cells[0].n, [None, Some(1), Some(3), None]);
        assert_eq!(grid.cells[1].n, [None, Some(2), Some(4), Some(0)]);
        assert_eq!(grid.cells[2].n, [None, None, Some(5), Some(1)]);
        assert_eq!(grid.cells[3].n, [Some(0), Some(4), Some(6), None]);
        assert_eq!(grid.cells[4].n, [Some(1), Some(5), Some(7), Some(3)]);
        assert_eq!(grid.cells[5].n, [Some(2), None, Some(8), Some(4)]);
        assert_eq!(grid.cells[6].n, [Some(3), Some(7), None, None]);
        assert_eq!(grid.cells[7].n, [Some(4), Some(8), None, Some(6)]);
        assert_eq!(grid.cells[8].n, [Some(5), None, None, Some(7)]);
    }
    #[test]
    fn rect_image() -> Result<(), image::ImageError> {
        crate::logger(log::LevelFilter::Trace);
        use crate::graphs::Graph;
        let mut grid: Orth<RectCell> = Orth::new(4, 4);

        grid.link(0, 1).unwrap();
        grid.link(1, 5).unwrap();
        grid.link(5, 6).unwrap();
        grid.link(6, 10).unwrap();
        grid.link(10, 14).unwrap();
        grid.link(14, 15).unwrap();

        grid.render().save("images/tests/rect_grid.png")
    }
}
