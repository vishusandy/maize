use super::RenderGraph;
use crate::algo::dist::Dist;
use crate::graphs::{Graph, Node};
use crate::render::opts;
use crate::render::state::graph;
use crate::render::RenderState;
use image::RgbaImage;
use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct State<'b, 'c, 'e, 'g, 'o, 'p, 'po, 'r, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
    pub(crate) state:
        std::borrow::Cow<'r, crate::render::state::graph::State<'b, 'c, 'e, 'g, 'o, G>>,
    pub(crate) dist: std::borrow::Cow<'p, Dist>,
    pub(crate) opts: std::borrow::Cow<'po, opts::DistOpts>,
}

impl<'b, 'c, 'e, 'g, 'o, 'p, 'po, 'r, G> State<'b, 'c, 'e, 'g, 'o, 'p, 'po, 'r, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
    pub fn render(&self) -> RgbaImage {
        self.render_image()
    }

    pub fn graph(&self) -> &crate::render::state::graph::State<'b, 'c, 'e, 'g, 'o, G> {
        &*self.state
    }

    pub fn dist(&self) -> &Dist {
        &*self.dist
    }

    pub fn opts(&self) -> &opts::DistOpts {
        &*self.opts
    }

    pub fn build_path(
        &'r self,
        start: usize,
        end: usize,
    ) -> Result<crate::render::state::path::BuilderState<'b, 'c, 'e, 'g, 'o, 'r, G>, crate::Error>
    {
        crate::Path::shortest_path(&*self.state.graph, &self.dist, end)
            .map(|path| crate::render::state::path::Builder::render_state(&*self.state))
    }
}

impl<'b, 'c, 'e, 'g, 'o, 'p, 'po, 'r, G> State<'b, 'c, 'e, 'g, 'o, 'p, 'po, 'r, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
}

impl<'b, 'c, 'e, 'g, 'o, 'p, 'po, 'r, G> RenderState<'b, 'c, 'e, 'g, 'o>
    for State<'b, 'c, 'e, 'g, 'o, 'p, 'po, 'r, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
    type Graph = G;

    fn render_image(&self) -> RgbaImage {
        let mut image = crate::render::new_image(
            &*self.state.graph,
            self.state.opts.size(),
            self.state.opts.colors(),
        );

        for cell in self.state.graph.cells() {
            self.fill(cell, &mut image);

            if self.state.opts.text().show() {
                if let Some(d) = self.dist.dist(cell.id()) {
                    let text = if self.opts.label_dist() {
                        d.to_string()
                    } else {
                        cell.id().to_string()
                    };
                    self.text(cell, &text, &mut image);
                }
            }
        }
        self.draw_edges(&mut image);
        image
    }

    fn fill(&self, cell: &<Self::Graph as Graph>::Node, image: &mut RgbaImage) {
        if let Some(d) = self.dist.dist(cell.id()) {
            self.state.graph.blend_fill(
                cell,
                &self.state.blocks[cell.id()],
                d,
                self.dist.max(),
                self.opts.bg(),
                image,
            )
        } else if let Some(color) = &self.state.node_state[cell.id()].color {
            self.state
                .graph
                .fill(cell, &self.state.blocks[cell.id()], color, image)
        }
    }

    fn text(&self, cell: &<Self::Graph as Graph>::Node, text: &str, image: &mut RgbaImage) {
        self.state.text(cell, text, image)
    }

    fn draw_edges(&self, image: &mut RgbaImage) {
        self.state.draw_edges(image)
    }
}

#[derive(Clone, Debug)]
pub struct Builder {}

impl Builder {
    pub fn render_state<'b, 'c, 'e, 'g, 'o, 'r, G>(
        state: &'r graph::State<'b, 'c, 'e, 'g, 'o, G>,
    ) -> BuilderState<'b, 'c, 'e, 'g, 'o, 'r, G>
    where
        G: RenderGraph + Clone + std::fmt::Debug,
        <<G as Graph>::Node as Node>::Block: Clone + std::fmt::Debug,
    {
        BuilderState {
            state: Cow::Borrowed(state),
        }
    }

    pub fn owned_render_state<'b, 'c, 'e, 'g, 'o, 'r, G>(
        state: graph::State<'b, 'c, 'e, 'g, 'o, G>,
    ) -> BuilderState<'b, 'c, 'e, 'g, 'o, 'r, G>
    where
        G: RenderGraph + Clone + std::fmt::Debug,
        <<G as Graph>::Node as Node>::Block: Clone + std::fmt::Debug,
    {
        BuilderState {
            state: Cow::Owned(state),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BuilderState<'b, 'c, 'e, 'g, 'o, 'r, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <<G as Graph>::Node as Node>::Block: Clone + std::fmt::Debug,
{
    state: Cow<'r, graph::State<'b, 'c, 'e, 'g, 'o, G>>,
}

impl<'b, 'c, 'e, 'g, 'o, 'r, G> BuilderState<'b, 'c, 'e, 'g, 'o, 'r, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <<G as Graph>::Node as Node>::Block: Clone + std::fmt::Debug,
{
    pub fn default_opts<'po>(self) -> BuilderOpts<'b, 'c, 'e, 'g, 'o, 'po, 'r, G> {
        BuilderOpts {
            state: self.state,
            opts: Cow::Owned(opts::DistOpts::default()),
        }
    }

    pub fn opts<'po>(
        self,
        opts: &'po opts::DistOpts,
    ) -> BuilderOpts<'b, 'c, 'e, 'g, 'o, 'po, 'r, G> {
        BuilderOpts {
            state: self.state,
            opts: Cow::Borrowed(opts),
        }
    }

    pub fn owned_opts<'po>(
        self,
        opts: opts::DistOpts,
    ) -> BuilderOpts<'b, 'c, 'e, 'g, 'o, 'po, 'r, G> {
        BuilderOpts {
            state: self.state,
            opts: Cow::Owned(opts),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BuilderOpts<'b, 'c, 'e, 'g, 'o, 'po, 'r, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <<G as Graph>::Node as Node>::Block: Clone + std::fmt::Debug,
{
    state: Cow<'r, graph::State<'b, 'c, 'e, 'g, 'o, G>>,
    opts: Cow<'po, opts::DistOpts>,
}

impl<'b, 'c, 'e, 'g, 'o, 'po, 'r, G> BuilderOpts<'b, 'c, 'e, 'g, 'o, 'po, 'r, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <<G as Graph>::Node as Node>::Block: Clone + std::fmt::Debug,
{
    pub fn simplified_dist<'pa>(
        self,
        start: usize,
    ) -> BuilderDist<'b, 'c, 'e, 'g, 'o, 'pa, 'po, 'r, G> {
        let dist = Dist::simple(&*self.state.graph, start);
        BuilderDist {
            state: self.state,
            dist: Cow::Owned(dist),
            opts: self.opts,
        }
    }

    pub fn dist<'pa>(self, dist: &'pa Dist) -> BuilderDist<'b, 'c, 'e, 'g, 'o, 'pa, 'po, 'r, G> {
        BuilderDist {
            state: self.state,
            dist: Cow::Borrowed(dist),
            opts: self.opts,
        }
    }

    pub fn owned_dist<'pa>(self, dist: Dist) -> BuilderDist<'b, 'c, 'e, 'g, 'o, 'pa, 'po, 'r, G> {
        BuilderDist {
            state: self.state,
            dist: Cow::Owned(dist),
            opts: self.opts,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BuilderDist<'b, 'c, 'e, 'g, 'o, 'pa, 'po, 'r, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <<G as Graph>::Node as Node>::Block: Clone + std::fmt::Debug,
{
    state: Cow<'r, graph::State<'b, 'c, 'e, 'g, 'o, G>>,
    dist: Cow<'pa, Dist>,
    opts: Cow<'po, opts::DistOpts>,
}

impl<'b, 'c, 'e, 'g, 'o, 'pa, 'po, 'r, G> BuilderDist<'b, 'c, 'e, 'g, 'o, 'pa, 'po, 'r, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <<G as Graph>::Node as Node>::Block: Clone + std::fmt::Debug,
{
    pub fn finish(self) -> State<'b, 'c, 'e, 'g, 'o, 'pa, 'po, 'r, G> {
        State {
            state: self.state,
            dist: self.dist,
            opts: self.opts,
        }
    }
}
