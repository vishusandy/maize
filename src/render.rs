pub(crate) mod opts;
pub(crate) mod path;
pub(crate) mod state;
use rusttype::Font;

use crate::graphs::{Graph, Node};
use image::{Rgba, RgbaImage};

const DEJAVU_BYTES: &[u8] = include_bytes!("../assets/DejaVuSansMono.ttf");

lazy_static::lazy_static! {
    pub(crate) static ref DEJAVU: Font<'static> = Font::try_from_bytes(DEJAVU_BYTES).unwrap();
}
// RenderBlock does NOT get implemented for Circular graph blocks
pub(crate) trait RenderBlock: Node {
    fn fill(&self, block: &Self::Block, color: &Rgba<u8>, img: &mut RgbaImage);
    fn edge(&self, block: &Self::Block, n: usize, color: &Rgba<u8>, img: &mut RgbaImage);
    fn text_pos(&self, block: &Self::Block, center: bool) -> (usize, usize);
    // draw functions go here
}
pub(crate) trait RenderGraph: Graph {
    fn render<'b, 'c, 'e, 'g, 'o>(
        &'g self,
        opts: std::borrow::Cow<'o, crate::render::opts::Basic>,
    ) -> crate::render::state::GraphState<'b, 'c, 'e, 'g, 'o, Self>
    where
        Self: Clone + std::fmt::Debug,
        <Self::Node as Node>::Block: Clone + std::fmt::Debug,
    {
        crate::render::state::GraphState::new(self, self.blocks(), opts)
    }
    fn size(&self, block_height: u32, block_width: u32, padding: u32) -> (u32, u32);
    fn blocks(&self) -> Vec<<Self::Node as Node>::Block>;
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
    ) -> (usize, usize);
    fn edge(
        &self,
        cell: &Self::Node,
        block: &<Self::Node as Node>::Block,
        n: usize,
        color: &Rgba<u8>,
        img: &mut RgbaImage,
    );
}

trait RenderState<'b, 'c, 'e, 'g, 'o> {
    type Graph: Graph;

    fn save<P>(&self, path: P) -> Result<(), image::ImageError>
    where
        P: AsRef<std::path::Path>,
    {
        self.render().save(path)
    }
    fn new(
        graph: &'g Self::Graph,
        blocks: Vec<<<Self::Graph as Graph>::Node as Node>::Block>,
        opts: std::borrow::Cow<'o, opts::Basic>,
    ) -> Self
    where
        Self::Graph: RenderGraph + Clone + std::fmt::Debug,
        <<Self::Graph as Graph>::Node as Node>::Block: Clone + std::fmt::Debug;
    fn borrowed(
        graph: &'g Self::Graph,
        blocks: &'b Vec<<<Self::Graph as Graph>::Node as Node>::Block>,
        node_state: &'c Vec<crate::render::state::NodeState>,
        edges: &'e crate::edges::Undirected<Rgba<u8>>,
        opts: &'o opts::Basic,
    ) -> Self;
    fn render(&self) -> RgbaImage;
    fn fill(&self, cell: &<Self::Graph as Graph>::Node, img: &mut RgbaImage);
    fn text(&self, cell: &<Self::Graph as Graph>::Node, text: &str, img: &mut RgbaImage);
    fn edges(&self, img: &mut RgbaImage);
}
