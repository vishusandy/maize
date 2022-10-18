mod builder;

use super::NodeState;
use super::{RenderGraph, RenderState};
use crate::edges::Undirected;
use crate::graphs::{Graph, Node};
pub(crate) use builder::{Builder, BuilderGraph};
use image::{Rgba, RgbaImage};
use std::borrow::Cow;

/// GraphState drawing operations should operate only on Graphs and NOT Nodes.
/// Graphs should specify the final operations while allowing Nodes to define
/// default operations.
#[derive(Clone, Debug)]
pub struct State<'b, 'c, 'e, 'g, 'o, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
    pub(super) graph: std::borrow::Cow<'g, G>,
    pub(super) blocks: std::borrow::Cow<'b, Vec<<<G as Graph>::Node as Node>::Block>>,
    pub(super) node_state: std::borrow::Cow<'c, Vec<NodeState>>,
    pub(super) edges: std::borrow::Cow<'e, Undirected<Rgba<u8>>>,
    pub(super) opts: std::borrow::Cow<'o, crate::render::opts::GraphOpts>,
}

impl<'b, 'c, 'e, 'g, 'o, G> State<'b, 'c, 'e, 'g, 'o, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
    pub fn render(&self) -> RgbaImage {
        self.render_image()
    }

    pub fn build_distance<'r>(
        &'r self,
        start: usize,
    ) -> crate::render::state::dist::BuilderState<'b, 'c, 'e, 'g, 'o, 'r, G> {
        crate::render::state::dist::Builder::render_state(self)
    }

    pub fn build_path<'r>(
        &'r self,
        start: usize,
        end: usize,
    ) -> Result<crate::render::state::path::BuilderState<'b, 'c, 'e, 'g, 'o, 'r, G>, crate::Error>
    {
        let dist = crate::Dist::simple(&*self.graph, start);
        crate::Path::shortest_path(&*self.graph, &dist, end)
            .map(|path| crate::render::state::path::Builder::render_state(self))
    }

    pub fn graph(&self) -> &G {
        &*self.graph
    }

    pub fn graph_mut(&mut self) -> &mut G {
        self.graph.to_mut()
    }

    pub fn opts(&self) -> &crate::render::opts::GraphOpts {
        &*self.opts
    }

    pub fn set_opts(&mut self, opts: Cow<'o, crate::render::opts::GraphOpts>) {
        self.opts = opts;
    }

    pub fn blocks(&self) -> &Vec<<<G as Graph>::Node as Node>::Block> {
        &*self.blocks
    }

    pub(crate) fn set_blocks(&mut self, blocks: Cow<'b, Vec<<<G as Graph>::Node as Node>::Block>>) {
        self.blocks = blocks;
    }

    pub fn node_state(&self) -> &Vec<NodeState> {
        &*self.node_state
    }

    pub(crate) fn set_node_state(&mut self, node_state: Cow<'c, Vec<NodeState>>) {
        self.node_state = node_state;
    }

    pub fn edges(&self) -> &Undirected<Rgba<u8>> {
        &*self.edges
    }

    pub(crate) fn set_edges(&mut self, edges: Cow<'e, Undirected<Rgba<u8>>>) {
        self.edges = edges;
    }

    pub(crate) fn set_edge(&mut self, id: usize, n: usize, color: Rgba<u8>) {
        let _ = (self.edges).to_mut().set_edge_value(id, n, color);
    }

    pub(crate) fn bg(&mut self, id: usize) -> Option<Rgba<u8>> {
        self.node_state[id].get()
    }

    pub(crate) fn set_bg(&mut self, id: usize, color: Option<Rgba<u8>>) {
        self.node_state.to_mut()[id].set(color);
    }
}

impl<'b, 'c, 'e, 'g, 'o, G> RenderState<'b, 'c, 'e, 'g, 'o> for State<'b, 'c, 'e, 'g, 'o, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
    type Graph = G;

    fn render_image(&self) -> RgbaImage {
        let mut image =
            crate::render::new_image(&*self.graph, self.opts.size(), self.opts.colors());

        for cell in self.graph.nodes() {
            self.fill(cell, &mut image);
            if self.opts.text().show() {
                let id = cell.id().to_string();
                self.text(cell, &id, &mut image);
            }
        }

        self.draw_edges(&mut image);
        image
    }

    fn fill(&self, cell: &<Self::Graph as Graph>::Node, image: &mut RgbaImage) {
        if let Some(color) = &self.node_state[cell.id()].color {
            self.graph.fill(cell, &self.blocks[cell.id()], color, image)
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

    fn draw_edges(&self, image: &mut RgbaImage) {
        for edge in self.edges.iter() {
            let id = edge.a().id();
            self.graph.edge(
                self.graph.node(id),
                &self.blocks[id],
                edge.a().side(),
                self.opts.size().dash_width(),
                edge.value(),
                self.opts.colors().dashed_edges(),
                image,
            );
        }

        self.edges.iter_outer().for_each(|(conn, col)| {
            let id = conn.id();
            self.graph.edge(
                self.graph.node(id),
                &self.blocks[id],
                conn.side(),
                self.opts.size().dash_width(),
                self.opts.colors().outer_edges(),
                self.opts.colors().dashed_edges(),
                image,
            );
        });
    }

    fn size(&self) -> (u32, u32) {
        self.graph.size(
            self.opts.size().block_height(),
            self.opts.size().block_height(),
            self.opts.size().padding(),
        )
        // size(&self, block_height: u32, block_width: u32, padding: u32)
    }
}
