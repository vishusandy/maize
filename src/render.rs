pub mod opts;
pub(crate) mod path;
pub(crate) mod state;
use freehand::Pt;
use rusttype::Font;

use crate::graphs::{Graph, Node};
use image::{Rgba, RgbaImage};

pub(crate) const DEJAVU_BYTES: &[u8] = include_bytes!("../assets/DejaVuSansMono.ttf");

lazy_static::lazy_static! {
    pub(crate) static ref DEJAVU: Font<'static> = Font::try_from_bytes(DEJAVU_BYTES).unwrap();
}
// RenderBlock does NOT get implemented for Circular graph blocks
pub(crate) trait RenderBlock: Node {
    fn block(&self, height: u32, width: u32, padding: u32) -> Self::Block;
    fn fill(&self, block: &Self::Block, color: &Rgba<u8>, img: &mut RgbaImage);
    fn edge_unlinked(&self, block: &Self::Block, n: usize, color: &Rgba<u8>, img: &mut RgbaImage);
    fn edge_linked(
        &self,
        block: &Self::Block,
        n: usize,
        width: u32,
        color: &Rgba<u8>,
        img: &mut RgbaImage,
    );
    fn text_pos(&self, block: &Self::Block, center: bool, padding: Pt<i32>) -> Pt<u32>;
    // draw functions go here
}
pub trait RenderGraph: Graph {
    fn size(&self, block_height: u32, block_width: u32, padding: u32) -> (u32, u32);
    fn blocks(
        &self,
        block_height: u32,
        block_width: u32,
        padding: u32,
    ) -> Vec<<Self::Node as Node>::Block>;
    fn fill(
        &self,
        cell: &Self::Node,
        block: &<Self::Node as Node>::Block,
        color: &Rgba<u8>,
        img: &mut RgbaImage,
    );
    fn text_pos(
        &self,
        cell: &Self::Node,
        block: &<Self::Node as Node>::Block,
        center: bool,
        padding: Pt<i32>,
    ) -> Pt<u32>;
    fn edge(
        &self,
        cell: &Self::Node,
        block: &<Self::Node as Node>::Block,
        n: usize,
        dash_width: u32,
        linked_color: &Rgba<u8>,
        unlinked_color: &Rgba<u8>,
        img: &mut RgbaImage,
    );
}

pub(crate) trait RenderState<'b, 'c, 'e, 'g, 'o> {
    type Graph: Graph;

    fn save<P>(&self, path: P) -> Result<(), image::ImageError>
    where
        P: AsRef<std::path::Path>,
    {
        self.render_image().save(path)
    }
    fn borrowed(
        graph: &'g Self::Graph,
        blocks: &'b Vec<<<Self::Graph as Graph>::Node as Node>::Block>,
        node_state: &'c Vec<crate::render::state::NodeState>,
        edges: &'e crate::edges::Undirected<Rgba<u8>>,
        opts: &'o opts::Basic,
    ) -> Self;
    fn render_image(&self) -> RgbaImage;
    fn fill(&self, cell: &<Self::Graph as Graph>::Node, img: &mut RgbaImage);
    fn text(&self, cell: &<Self::Graph as Graph>::Node, text: &str, img: &mut RgbaImage);
    fn edges(&self, img: &mut RgbaImage);
}
