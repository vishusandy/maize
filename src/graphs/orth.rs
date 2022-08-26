pub(crate) mod nodes;

use super::{Graph, Node};
use crate::render::{RenderBlock, RenderGraph};
use image::{Rgba, RgbaImage};

#[derive(Clone, Debug)]
struct Orth<C: Node> {
    _phantom: std::marker::PhantomData<C>,
}
impl<C: Node> Graph for Orth<C> {
    type Node = C;
    fn cell(&self, _id: usize) -> Self::Node {
        todo!()
    }
    fn len(&self) -> usize {
        todo!()
    }
    fn cell_mut(&mut self, id: usize) -> &mut Self::Node {
        todo!()
    }
    fn cells(&self) -> Box<dyn Iterator<Item = &Self::Node> + '_> {
        todo!()
    }
}

impl<C: RenderBlock> RenderGraph for Orth<C> {
    fn size(&self, _block_height: u32, _block_width: u32, _padding: u32) -> (u32, u32) {
        todo!()
    }
    fn blocks(&self) -> Vec<<Self::Node as Node>::Block> {
        todo!()
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
    fn text_pos(
        &self,
        cell: &Self::Node,
        _block: &<Self::Node as Node>::Block,
        center: bool,
    ) -> (usize, usize) {
        cell.text_pos(_block, center)
    }
    fn edge(
        &self,
        cell: &Self::Node,
        block: &<Self::Node as Node>::Block,
        n: usize,
        color: &Rgba<u8>,
        img: &mut RgbaImage,
    ) {
        todo!()
    }
}
