use crate::graphs::{Block, Node};
use image::{Rgba, RgbaImage};

#[derive(Clone, Debug)]
pub(crate) struct RectBlock {}
impl Block for RectBlock {}

#[derive(Clone, Debug)]
pub(crate) struct RectCell {}
impl Node for RectCell {
    type Block = RectBlock;
    const N: usize = 4;
    fn id(&self) -> usize {
        todo!()
    }
    fn all_neighbors(&self) -> &[Option<usize>] {
        todo!()
    }
    fn neighbors(&self) -> crate::graphs::Neighbors<Self>
    where
        Self: Sized,
    {
        todo!()
    }
    fn num_neighbors(&self) -> usize {
        todo!()
    }
    fn neighbor(&self, n: usize) -> Option<usize> {
        todo!()
    }
    fn neighbor_id(&self, cell: usize) -> Option<usize> {
        todo!()
    }
}

impl crate::render::RenderBlock for RectCell {
    fn fill(&self, _block: &Self::Block, _color: &Rgba<u8>, _img: &mut RgbaImage) {}
    fn edge(&self, _block: &Self::Block, _n: usize, _color: &Rgba<u8>, _img: &mut RgbaImage) {}
    fn text_pos(&self, _block: &Self::Block, center: bool) -> (usize, usize) {
        todo!()
    }
}
