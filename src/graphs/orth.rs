pub(crate) mod nodes;

use super::{Graph, Node};
use crate::render::{RenderBlock, RenderGraph};
use image::{Rgba, RgbaImage};

#[derive(Clone, Debug)]
pub struct Orth<C: Node> {
    len: usize,
    height: usize,
    width: usize,
    cells: Vec<C>,
}
impl<C: Node> Orth<C> {
    fn check_id(&self, cell: usize) -> Result<(), crate::Error> {
        match cell >= self.len {
            true => Err(crate::Error::InvalidCell(cell, self.len)),
            false => Ok(()),
        }
    }
}
impl<C: Node> Graph for Orth<C> {
    type Node = C;
    fn len(&self) -> usize {
        self.len
    }
    fn cell(&self, id: usize) -> &Self::Node {
        &self.cells[id]
    }
    fn cell_mut(&mut self, id: usize) -> &mut Self::Node {
        &mut self.cells[id]
    }
    fn cells(&self) -> Box<dyn Iterator<Item = &Self::Node> + '_> {
        Box::new(Iter::new(self))
    }
    fn link(&mut self, a: usize, b: usize) -> Result<(), crate::Error> {
        if a >= self.len {
            return Err(crate::Error::InvalidCell(a, self.len));
        }
        if b >= self.len {
            return Err(crate::Error::InvalidCell(b, self.len));
        }
        self.cells[a].link(b).and_then(|_| self.cells[b].link(a))
    }
    fn unlink(&mut self, a: usize, b: usize) -> Result<(), crate::Error> {
        self.check_id(a)
            .and_then(|_| self.check_id(b))
            .and_then(|_| {
                self.cells[a]
                    .unlink(b)
                    .and_then(|_| self.cells[b].unlink(a))
            })
    }
}

pub struct Iter<'a, C: Node> {
    slice: &'a [C],
}
impl<'a, C: Node> Iter<'a, C> {
    fn new(grid: &'a Orth<C>) -> Self {
        Self {
            slice: &grid.cells[..],
        }
    }
}

impl<'a, C: Node> Iterator for Iter<'a, C> {
    type Item = &'a C;
    fn next(&mut self) -> Option<Self::Item> {
        match self.slice {
            [first, rest @ ..] => {
                self.slice = rest;
                Some(first)
            }
            [] => None,
        }
    }
}

impl<C: RenderBlock> RenderGraph for Orth<C> {
    fn size(&self, block_height: u32, block_width: u32, padding: u32) -> (u32, u32) {
        (
            (self.width) as u32 * (block_width + 1) + padding + padding,
            (self.height) as u32 * (block_height + 1) + padding + padding,
        )
    }
    fn blocks(&self, height: u32, width: u32, padding: u32) -> Vec<<Self::Node as Node>::Block> {
        self.cells
            .iter()
            .map(|c| c.block(height, width, padding))
            .collect()
    }
    fn fill(
        &self,
        cell: &Self::Node,
        block: &<Self::Node as Node>::Block,
        color: &Rgba<u8>,
        img: &mut RgbaImage,
    ) {
        cell.fill(block, color, img);
    }
    fn blend_fill(
        &self,
        cell: &Self::Node,
        block: &<Self::Node as Node>::Block,
        i: usize,
        max: usize,
        blend: &crate::render::opts::blend::Blend,
        image: &mut RgbaImage,
    ) {
        cell.blend_fill(block, i, max, blend, image);
    }
    fn text_pos(
        &self,
        cell: &Self::Node,
        _block: &<Self::Node as Node>::Block,
        center: bool,
        padding: freehand::Pt<i32>,
    ) -> freehand::Pt<u32> {
        cell.text_pos(_block, center, padding)
    }
    fn edge(
        &self,
        cell: &Self::Node,
        block: &<Self::Node as Node>::Block,
        n: usize,
        dash_width: u32,
        unlinked_color: &Rgba<u8>,
        linked_color: &Rgba<u8>,
        image: &mut RgbaImage,
    ) {
        if !cell.linked_side(n) {
            cell.edge_unlinked(block, n, unlinked_color, image);
        } else if dash_width != 0 {
            cell.edge_linked(block, n, dash_width, linked_color, image);
        }
    }
    fn arrow(
        &self,
        cell: &Self::Node,
        block: &<Self::Node as Node>::Block,
        from_n: usize,
        to_n: usize,
        style: &crate::render::opts::Arrow,
        color: Rgba<u8>,
        image: &mut RgbaImage,
    ) {
        cell.arrow(block, from_n, to_n, style, color, image);
    }
    fn half_arrow(
        &self,
        cell: &Self::Node,
        block: &<Self::Node as Node>::Block,
        n: usize,
        style: &crate::render::opts::Arrow,
        color: Rgba<u8>,
        image: &mut RgbaImage,
    ) {
        cell.half_arrow(block, n, style, color, image);
    }
}
