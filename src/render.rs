pub mod opts;
pub(crate) mod state;
use freehand::Pt;
use rusttype::Font;

use crate::graphs::{Graph, Node};
use image::{Rgba, RgbaImage};

pub(crate) const DEJAVU_BYTES: &[u8] = include_bytes!("../assets/DejaVuSansMono.ttf");

lazy_static::lazy_static! {
    pub(crate) static ref DEJAVU: Font<'static> = Font::try_from_bytes(DEJAVU_BYTES).unwrap();
}
/// Optional trait for nodes that can be rendered with only a block - no additional information from the graph required
// RenderBlock does NOT get implemented for Circular graph blocks
pub(crate) trait RenderBlock: Node {
    fn block(&self, height: u32, width: u32, padding: u32) -> Self::Block;
    fn fill(&self, block: &Self::Block, color: &Rgba<u8>, img: &mut RgbaImage);
    fn blend_fill(
        &self,
        block: &Self::Block,
        i: usize,
        max: usize,
        blend: &crate::render::opts::blend::Blend,
        image: &mut RgbaImage,
    );
    /// Draw a solid edge for edges with no link
    fn edge_unlinked(&self, block: &Self::Block, n: usize, color: &Rgba<u8>, img: &mut RgbaImage);
    /// Draw a dashed edge for edges with a link
    fn edge_linked(
        &self,
        block: &Self::Block,
        n: usize,
        dash_width: u32,
        color: &Rgba<u8>,
        image: &mut RgbaImage,
    );
    fn text_pos(&self, block: &Self::Block, center: bool, padding: Pt<i32>) -> Pt<u32>;
    fn arrow(
        &self,
        block: &Self::Block,
        from_n: usize,
        to_n: usize,
        style: &crate::render::opts::Arrow,
        color: Rgba<u8>,
        image: &mut RgbaImage,
    );
    fn half_arrow(
        &self,
        block: &Self::Block,
        n: usize,
        style: &crate::render::opts::Arrow,
        color: Rgba<u8>,
        image: &mut RgbaImage,
    );
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
        image: &mut RgbaImage,
    );
    fn blend_fill(
        &self,
        cell: &Self::Node,
        block: &<Self::Node as Node>::Block,
        i: usize,
        max: usize,
        blend: &crate::render::opts::blend::Blend,
        image: &mut RgbaImage,
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
        image: &mut RgbaImage,
    );
    fn arrow(
        &self,
        cell: &Self::Node,
        block: &<Self::Node as Node>::Block,
        from_n: usize,
        to_n: usize,
        style: &crate::render::opts::Arrow,
        color: Rgba<u8>,
        image: &mut RgbaImage,
    );
    fn half_arrow(
        &self,
        cell: &Self::Node,
        block: &<Self::Node as Node>::Block,
        n: usize,
        style: &crate::render::opts::Arrow,
        color: Rgba<u8>,
        image: &mut RgbaImage,
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
    fn render_image(&self) -> RgbaImage;
    fn fill(&self, cell: &<Self::Graph as Graph>::Node, img: &mut RgbaImage);
    fn text(&self, cell: &<Self::Graph as Graph>::Node, text: &str, img: &mut RgbaImage);
    fn edges(&self, img: &mut RgbaImage);
}

pub(crate) fn new_image<G: RenderGraph>(
    graph: &G,
    size: &opts::Size,
    colors: &opts::Colors,
) -> RgbaImage {
    let (x, y) = graph.size(size.block_height(), size.block_width(), size.padding());
    let mut image = RgbaImage::from_pixel(x, y, colors.image_bg());
    if let Some(bg) = colors.maze_bg() {
        let pad = size.padding() as i32;
        let pad2 = size.padding() + size.padding();
        let rect = imageproc::rect::Rect::at(pad, pad).of_size(x - pad2, y - pad2);
        imageproc::drawing::draw_filled_rect_mut(&mut image, rect, bg);
    }
    image
}
