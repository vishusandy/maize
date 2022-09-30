use super::{RenderGraph, RenderState};
use crate::edges::Undirected;
use crate::graphs::{Graph, Node};
use image::{Rgba, RgbaImage};
use std::borrow::Cow;

#[derive(Clone, Debug)]
pub(crate) struct NodeState {
    color: Option<Rgba<u8>>,
}
impl NodeState {
    fn new(color: &Rgba<u8>) -> Self {
        Self {
            color: Some(color.clone()),
        }
    }
    fn vec(len: usize, color: &Rgba<u8>) -> Vec<Self> {
        (0..len).map(|_| Self::new(color)).collect()
    }
}

/// GraphState drawing operations should operate only on Graphs and NOT Nodes - Graphs should specify
/// the final operations while allowing Nodes to define default operations.
#[derive(Clone, Debug)]
pub(crate) struct GraphState<'b, 'c, 'e, 'g, 'o, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
    // don't store image here - create it in the render method
    graph: std::borrow::Cow<'g, G>,
    blocks: std::borrow::Cow<'b, Vec<<<G as Graph>::Node as Node>::Block>>,
    node_state: std::borrow::Cow<'c, Vec<NodeState>>,
    edges: std::borrow::Cow<'e, Undirected<Rgba<u8>>>,
    opts: std::borrow::Cow<'o, crate::render::opts::Basic>,
}

impl<'b, 'c, 'e, 'g, 'o, G> RenderState<'b, 'c, 'e, 'g, 'o> for GraphState<'b, 'c, 'e, 'g, 'o, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
    type Graph = G;

    fn new(
        graph: &'g Self::Graph,
        blocks: Vec<<<Self::Graph as Graph>::Node as Node>::Block>,
        opts: Cow<'o, crate::render::opts::Basic>,
    ) -> Self
    where
        Self::Graph: RenderGraph + Clone + std::fmt::Debug,
        <<Self::Graph as Graph>::Node as Node>::Block: Clone + std::fmt::Debug,
    {
        Self {
            graph: std::borrow::Cow::Borrowed(graph),
            blocks: std::borrow::Cow::Owned(blocks),
            node_state: std::borrow::Cow::Owned(NodeState::vec(
                graph.len(),
                &opts.colors().cell_bg(),
            )),
            edges: Cow::Owned(crate::edges::Undirected::new(graph, *opts.colors().edges())),
            opts,
        }
    }

    fn borrowed(
        graph: &'g Self::Graph,
        blocks: &'b Vec<<<Self::Graph as Graph>::Node as Node>::Block>,
        node_state: &'c Vec<crate::render::state::NodeState>,
        edges: &'e crate::edges::Undirected<Rgba<u8>>,
        opts: &'o crate::render::opts::Basic,
    ) -> Self {
        Self {
            graph: Cow::Borrowed(graph),
            blocks: Cow::Borrowed(blocks),
            node_state: Cow::Borrowed(node_state),
            edges: Cow::Borrowed(edges),
            opts: Cow::Borrowed(opts),
        }
    }

    fn render(&self) -> RgbaImage {
        let size = self.opts.size();
        let (x, y) = self
            .graph
            .size(size.block_height(), size.block_width(), size.padding());
        #[allow(unused_mut)]
        let mut img = RgbaImage::from_pixel(x, y, self.opts.colors().maze_bg());
        for cell in self.graph.cells() {
            self.fill(cell, &mut img);
            if self.opts.text().show() {
                let id = cell.id().to_string();
                self.text(cell, &id, &mut img);
            }
        }
        self.edges(&mut img);
        img
    }

    fn fill(&self, cell: &<Self::Graph as Graph>::Node, img: &mut RgbaImage) {
        if let Some(color) = &self.node_state[cell.id()].color {
            self.graph.fill(cell, &self.blocks[cell.id()], color, img)
        }
    }

    fn text(&self, cell: &<Self::Graph as Graph>::Node, text: &str, image: &mut RgbaImage) {
        use imageproc::drawing::{draw_text_mut, text_size};
        let padding = if self.opts.text().center() {
            let size = freehand::Pt::from(text_size(
                self.opts.text().scale(),
                &crate::render::DEJAVU,
                text,
            ));
            self.opts.text().padding() - (size.div(2))
        } else {
            self.opts.text().padding()
        };
        let pt = self.graph.text_pos(
            cell,
            &self.blocks[cell.id()],
            self.opts.text().center(),
            padding,
        );
        draw_text_mut(
            image,
            *self.opts.colors().text(),
            pt.x() as i32,
            pt.y() as i32,
            self.opts.text().scale(),
            &crate::render::DEJAVU,
            text,
        );
    }

    fn edges(&self, img: &mut RgbaImage) {
        for edge in self.edges.edges() {
            let id = edge.a().id();
            let side = edge.a().side();
            let color = edge.value();
            self.graph.edge(
                &self.graph.cell(id),
                &self.blocks[id],
                side,
                self.opts.size().dash_width(),
                color,
                self.opts.colors().dashed_edges(),
                img,
            );
        }
    }
}
