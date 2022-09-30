use super::{RenderGraph, RenderState};
use crate::edges::Undirected;
use crate::graphs::{Graph, Node};
use crate::render::opts::Basic;
use image::{Rgba, RgbaImage};
use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct NodeState {
    color: Option<Rgba<u8>>,
}
impl NodeState {
    pub fn new(color: &Rgba<u8>) -> Self {
        Self {
            color: Some(*color),
        }
    }
    pub fn vec(len: usize, color: &Rgba<u8>) -> Vec<Self> {
        (0..len).map(|_| Self::new(color)).collect()
    }
}

/// GraphState drawing operations should operate only on Graphs and NOT Nodes - Graphs should specify
/// the final operations while allowing Nodes to define default operations.
#[derive(Clone, Debug)]
pub struct GraphState<'b, 'c, 'e, 'g, 'o, G>
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

impl<'b, 'c, 'e, 'g, 'o, G> GraphState<'b, 'c, 'e, 'g, 'o, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
    pub fn render(&self) -> RgbaImage {
        self.render_image()
    }
}

impl<'b, 'c, 'e, 'g, 'o, G> RenderState<'b, 'c, 'e, 'g, 'o> for GraphState<'b, 'c, 'e, 'g, 'o, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
    type Graph = G;
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

    fn render_image(&self) -> RgbaImage {
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

    fn edges(&self, image: &mut RgbaImage) {
        for edge in self.edges.iter() {
            let id = edge.a().id();
            self.graph.edge(
                &self.graph.cell(id),
                &self.blocks[id],
                edge.a().side(),
                self.opts.size().dash_width(),
                edge.value(),
                self.opts.colors().dashed_edges(),
                image,
            );
        }
        #[cfg(test)]
        log::debug!("outer edges");
        self.edges.iter_outer().for_each(|(conn, col)| {
            let id = conn.id();
            #[cfg(test)]
            log::debug!("id={} side={}", id, conn.side());
            self.graph.edge(
                &self.graph.cell(id),
                &self.blocks[id],
                conn.side(),
                self.opts.size().dash_width(),
                self.opts.colors().outer_edges(),
                self.opts.colors().dashed_edges(),
                image,
            );
        });
    }
}

#[derive(Clone, Debug)]
pub struct GraphStateBuilder {}

impl GraphStateBuilder {
    pub fn owned_graph<'g, G>(grid: G) -> BuilderGraph<'g, G>
    where
        G: RenderGraph + Clone + std::fmt::Debug,
        <G::Node as Node>::Block: Clone + std::fmt::Debug,
    {
        BuilderGraph {
            graph: Cow::Owned(grid),
        }
    }
    pub fn graph<'g, G>(grid: &'g G) -> BuilderGraph<'g, G>
    where
        G: RenderGraph + Clone + std::fmt::Debug,
        <G::Node as Node>::Block: Clone + std::fmt::Debug,
    {
        BuilderGraph {
            graph: Cow::Borrowed(grid),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BuilderGraph<'g, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
    graph: Cow<'g, G>,
}

impl<'g, G> BuilderGraph<'g, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
    pub fn default_opts<'o>(self) -> BuilderOpts<'g, 'o, G> {
        BuilderOpts {
            graph: self.graph,
            opts: Cow::Owned(Basic::default()),
        }
    }
    pub fn owned_opts<'o>(self, opts: Basic) -> BuilderOpts<'g, 'o, G> {
        BuilderOpts {
            graph: self.graph,
            opts: Cow::Owned(opts),
        }
    }
    pub fn opts<'o>(self, opts: &'o Basic) -> BuilderOpts<'g, 'o, G> {
        BuilderOpts {
            graph: self.graph,
            opts: Cow::Borrowed(opts),
        }
    }
    pub fn finish<'b, 'c, 'e, 'o>(self) -> GraphState<'b, 'c, 'e, 'g, 'o, G> {
        self.default_opts()
            .default_blocks()
            .default_nodes()
            .default_edges()
            .build()
    }
}

#[derive(Clone, Debug)]
pub struct BuilderOpts<'g, 'o, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
    graph: std::borrow::Cow<'g, G>,
    opts: std::borrow::Cow<'o, Basic>,
}
impl<'g, 'o, G> BuilderOpts<'g, 'o, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
    pub fn owned_blocks<'b>(
        self,
        blocks: Vec<<G::Node as Node>::Block>,
    ) -> BuilderBlocks<'b, 'g, 'o, G> {
        BuilderBlocks {
            graph: self.graph,
            blocks: Cow::Owned(blocks),
            opts: self.opts,
        }
    }
    pub fn blocks<'b>(
        self,
        blocks: &'b Vec<<G::Node as Node>::Block>,
    ) -> BuilderBlocks<'b, 'g, 'o, G> {
        BuilderBlocks {
            graph: self.graph,
            blocks: Cow::Borrowed(blocks),
            opts: self.opts,
        }
    }
    pub fn default_blocks<'b>(self) -> BuilderBlocks<'b, 'g, 'o, G> {
        let blocks = self.graph.blocks(
            self.opts.size().block_height(),
            self.opts.size().block_width(),
            self.opts.size().padding(),
        );
        BuilderBlocks {
            graph: self.graph,
            blocks: Cow::Owned(blocks),
            opts: self.opts,
        }
    }
    pub fn finish<'b, 'c, 'e>(self) -> GraphState<'b, 'c, 'e, 'g, 'o, G> {
        self.default_blocks()
            .default_nodes()
            .default_edges()
            .build()
    }
}

#[derive(Clone, Debug)]
pub struct BuilderBlocks<'b, 'g, 'o, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
    graph: std::borrow::Cow<'g, G>,
    blocks: std::borrow::Cow<'b, Vec<<<G as Graph>::Node as Node>::Block>>,
    opts: std::borrow::Cow<'o, crate::render::opts::Basic>,
}
impl<'b, 'g, 'o, G> BuilderBlocks<'b, 'g, 'o, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
    pub fn owned_nodes<'c>(self, nodes: Vec<NodeState>) -> BuilderNodes<'b, 'c, 'g, 'o, G> {
        BuilderNodes {
            graph: self.graph,
            blocks: self.blocks,
            opts: self.opts,
            node_state: Cow::Owned(nodes),
        }
    }
    pub fn nodes<'c>(self, nodes: &'c Vec<NodeState>) -> BuilderNodes<'b, 'c, 'g, 'o, G> {
        BuilderNodes {
            graph: self.graph,
            blocks: self.blocks,
            opts: self.opts,
            node_state: Cow::Borrowed(nodes),
        }
    }
    pub fn default_nodes<'c>(self) -> BuilderNodes<'b, 'c, 'g, 'o, G> {
        let nodes = vec![NodeState::new(self.opts.colors().cell_bg()); self.graph.len()];
        BuilderNodes {
            graph: self.graph,
            blocks: self.blocks,
            opts: self.opts,
            node_state: Cow::Owned(nodes),
        }
    }
    pub fn finish<'c, 'e>(self) -> GraphState<'b, 'c, 'e, 'g, 'o, G> {
        self.default_nodes().default_edges().build()
    }
}

#[derive(Clone, Debug)]
pub struct BuilderNodes<'b, 'c, 'g, 'o, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
    graph: std::borrow::Cow<'g, G>,
    blocks: std::borrow::Cow<'b, Vec<<<G as Graph>::Node as Node>::Block>>,
    opts: std::borrow::Cow<'o, crate::render::opts::Basic>,
    node_state: std::borrow::Cow<'c, Vec<NodeState>>,
}
impl<'b, 'c, 'g, 'o, G> BuilderNodes<'b, 'c, 'g, 'o, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
    pub fn owned_edges<'e>(
        self,
        edges: Undirected<Rgba<u8>>,
    ) -> BuilderEdges<'b, 'c, 'e, 'g, 'o, G> {
        BuilderEdges {
            graph: self.graph,
            blocks: self.blocks,
            opts: self.opts,
            node_state: self.node_state,
            edges: Cow::Owned(edges),
        }
    }
    pub fn edges<'e>(self, edges: &'e Undirected<Rgba<u8>>) -> BuilderEdges<'b, 'c, 'e, 'g, 'o, G> {
        BuilderEdges {
            graph: self.graph,
            blocks: self.blocks,
            opts: self.opts,
            node_state: self.node_state,
            edges: Cow::Borrowed(edges),
        }
    }
    pub fn default_edges<'e>(self) -> BuilderEdges<'b, 'c, 'e, 'g, 'o, G> {
        let edges = Undirected::new(&*self.graph, *self.opts.colors().edges());
        BuilderEdges {
            graph: self.graph,
            blocks: self.blocks,
            opts: self.opts,
            node_state: self.node_state,
            edges: Cow::Owned(edges),
        }
    }
    pub fn finish<'e>(self) -> GraphState<'b, 'c, 'e, 'g, 'o, G> {
        self.default_edges().build()
    }
}

#[derive(Clone, Debug)]
pub struct BuilderEdges<'b, 'c, 'e, 'g, 'o, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
    graph: std::borrow::Cow<'g, G>,
    blocks: std::borrow::Cow<'b, Vec<<<G as Graph>::Node as Node>::Block>>,
    opts: std::borrow::Cow<'o, crate::render::opts::Basic>,
    node_state: std::borrow::Cow<'c, Vec<NodeState>>,
    edges: std::borrow::Cow<'e, Undirected<Rgba<u8>>>,
}
impl<'b, 'c, 'e, 'g, 'o, G> BuilderEdges<'b, 'c, 'e, 'g, 'o, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
    pub fn build(self) -> GraphState<'b, 'c, 'e, 'g, 'o, G> {
        GraphState {
            graph: self.graph,
            blocks: self.blocks,
            opts: self.opts,
            node_state: self.node_state,
            edges: self.edges,
        }
    }
}
