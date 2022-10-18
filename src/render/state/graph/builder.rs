use super::NodeState;
use super::RenderGraph;
use crate::edges::Undirected;
use crate::graphs::{Graph, Node};
use crate::render::opts::GraphOpts;
use image::Rgba;
use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct Builder {}

impl Builder {
    pub fn owned_graph<'g, G>(grid: G) -> BuilderGraph<'g, G>
    where
        G: RenderGraph + Clone + std::fmt::Debug,
        <G::Node as Node>::Block: Clone + std::fmt::Debug,
    {
        BuilderGraph {
            graph: Cow::Owned(grid),
        }
    }
    pub fn graph<G>(grid: &G) -> BuilderGraph<'_, G>
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
            opts: Cow::Owned(GraphOpts::default()),
        }
    }
    pub fn owned_opts<'o>(self, opts: GraphOpts) -> BuilderOpts<'g, 'o, G> {
        BuilderOpts {
            graph: self.graph,
            opts: Cow::Owned(opts),
        }
    }
    pub fn opts<'o>(self, opts: &'o GraphOpts) -> BuilderOpts<'g, 'o, G> {
        BuilderOpts {
            graph: self.graph,
            opts: Cow::Borrowed(opts),
        }
    }
    pub fn finish<'b, 'c, 'e, 'o>(self) -> State<'b, 'c, 'e, 'g, 'o, G> {
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
    opts: std::borrow::Cow<'o, GraphOpts>,
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
    pub fn finish<'b, 'c, 'e>(self) -> State<'b, 'c, 'e, 'g, 'o, G> {
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
    opts: std::borrow::Cow<'o, crate::render::opts::GraphOpts>,
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
    pub fn finish<'c, 'e>(self) -> State<'b, 'c, 'e, 'g, 'o, G> {
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
    opts: std::borrow::Cow<'o, crate::render::opts::GraphOpts>,
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
        let edges = Undirected::new(
            &*self.graph,
            *self.opts.colors().edges(),
            *self.opts.colors().outer_edges(),
        );
        BuilderEdges {
            graph: self.graph,
            blocks: self.blocks,
            opts: self.opts,
            node_state: self.node_state,
            edges: Cow::Owned(edges),
        }
    }
    pub fn finish<'e>(self) -> State<'b, 'c, 'e, 'g, 'o, G> {
        self.default_edges().build()
    }
}
use super::State;

#[derive(Clone, Debug)]
pub struct BuilderEdges<'b, 'c, 'e, 'g, 'o, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
    graph: std::borrow::Cow<'g, G>,
    blocks: std::borrow::Cow<'b, Vec<<<G as Graph>::Node as Node>::Block>>,
    opts: std::borrow::Cow<'o, crate::render::opts::GraphOpts>,
    node_state: std::borrow::Cow<'c, Vec<NodeState>>,
    edges: std::borrow::Cow<'e, Undirected<Rgba<u8>>>,
}
impl<'b, 'c, 'e, 'g, 'o, G> BuilderEdges<'b, 'c, 'e, 'g, 'o, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
    pub fn build(self) -> State<'b, 'c, 'e, 'g, 'o, G> {
        State {
            graph: self.graph,
            blocks: self.blocks,
            opts: self.opts,
            node_state: self.node_state,
            edges: self.edges,
        }
    }
}
