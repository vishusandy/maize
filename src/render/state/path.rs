use super::RenderGraph;
use crate::algo::path;
use crate::graphs::{Graph, Node};
use crate::render::opts;
use crate::render::state::graph;
use crate::render::RenderState;
use image::{Rgba, RgbaImage};
use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct State<'b, 'c, 'e, 'g, 'o, 'p, 'po, 'r, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
    state: std::borrow::Cow<'r, graph::State<'b, 'c, 'e, 'g, 'o, G>>,
    path: std::borrow::Cow<'p, path::Path>,
    opts: std::borrow::Cow<'po, opts::Path>,
}

impl<'b, 'c, 'e, 'g, 'o, 'p, 'po, 'r, G> State<'b, 'c, 'e, 'g, 'o, 'p, 'po, 'r, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
    pub fn render(&self) -> RgbaImage {
        self.render_image()
    }
}

impl<'b, 'c, 'e, 'g, 'o, 'p, 'po, 'r, G> State<'b, 'c, 'e, 'g, 'o, 'p, 'po, 'r, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
    fn arrows(&self, node: &G::Node, style: &opts::Arrow, color: Rgba<u8>, image: &mut RgbaImage) {
        if let Some(step) = self.path.step_num(node.id()) {
            let block = &self.state.blocks[node.id()];
            if step == 0 {
                if let Some(n) = node.neighbor_id(self.path.path[step + 1]) {
                    self.state
                        .graph
                        .half_arrow(node, block, n, style, color, image);
                }
            } else if step == self.path.path.len() - 1 {
                if let Some(n) = node.neighbor_id(self.path.path[step - 1]) {
                    self.state
                        .graph
                        .half_arrow(node, block, n, style, color, image);
                }
            } else {
                let prev = node.neighbor_id(self.path.path[step - 1]).unwrap();
                let next = node.neighbor_id(self.path.path[step + 1]).unwrap();
                self.state
                    .graph
                    .arrow(node, block, prev, next, style, color, image);
            }
        }
    }
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

            if let Some(arrow) = self.opts.arrows() {
                self.arrows(cell, self.opts.style(), arrow, &mut image);
            }

            if self.state.opts.text().show() {
                if let Some(step) = self.path.step_num(cell.id()) {
                    let text = if self.opts.label_steps() {
                        step.to_string()
                    } else {
                        cell.id().to_string()
                    };
                    self.text(cell, &text, &mut image);
                }
            }
        }

        self.edges(&mut image);
        image
    }
    fn fill(&self, cell: &<Self::Graph as Graph>::Node, image: &mut RgbaImage) {
        if let Some(step) = self.path.step_num(cell.id()) {
            self.state.graph.blend_fill(
                cell,
                &self.state.blocks[cell.id()],
                step,
                self.path.max.unwrap_or_else(|| self.path.path.len() - 1),
                self.opts.path_bg(),
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
    fn edges(&self, image: &mut RgbaImage) {
        self.state.edges(image)
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
            opts: Cow::Owned(opts::Path::default()),
        }
    }

    pub fn opts<'po>(self, opts: &'po opts::Path) -> BuilderOpts<'b, 'c, 'e, 'g, 'o, 'po, 'r, G> {
        BuilderOpts {
            state: self.state,
            opts: Cow::Borrowed(opts),
        }
    }

    pub fn owned_opts<'po>(self, opts: opts::Path) -> BuilderOpts<'b, 'c, 'e, 'g, 'o, 'po, 'r, G> {
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
    opts: Cow<'po, opts::Path>,
}

impl<'b, 'c, 'e, 'g, 'o, 'po, 'r, G> BuilderOpts<'b, 'c, 'e, 'g, 'o, 'po, 'r, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <<G as Graph>::Node as Node>::Block: Clone + std::fmt::Debug,
{
    pub fn simplified_path<'pa>(self) -> BuilderOpts<'b, 'c, 'e, 'g, 'o, 'po, 'r, G> {
        todo!()
    }

    pub fn path<'pa>(
        self,
        path: &'pa path::Path,
    ) -> BuilderPath<'b, 'c, 'e, 'g, 'o, 'pa, 'po, 'r, G> {
        BuilderPath {
            state: self.state,
            path: Cow::Borrowed(path),
            opts: self.opts,
        }
    }

    pub fn owned_path<'pa>(
        self,
        path: path::Path,
    ) -> BuilderPath<'b, 'c, 'e, 'g, 'o, 'pa, 'po, 'r, G> {
        BuilderPath {
            state: self.state,
            path: Cow::Owned(path),
            opts: self.opts,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BuilderPath<'b, 'c, 'e, 'g, 'o, 'pa, 'po, 'r, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <<G as Graph>::Node as Node>::Block: Clone + std::fmt::Debug,
{
    state: Cow<'r, graph::State<'b, 'c, 'e, 'g, 'o, G>>,
    path: Cow<'pa, path::Path>,
    opts: Cow<'po, opts::Path>,
}

impl<'b, 'c, 'e, 'g, 'o, 'pa, 'po, 'r, G> BuilderPath<'b, 'c, 'e, 'g, 'o, 'pa, 'po, 'r, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <<G as Graph>::Node as Node>::Block: Clone + std::fmt::Debug,
{
    pub fn finish(self) -> State<'b, 'c, 'e, 'g, 'o, 'pa, 'po, 'r, G> {
        State {
            state: self.state,
            path: self.path,
            opts: self.opts,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphs::orth::nodes::rect::RectCell;
    use crate::graphs::orth::Orth;
    #[test]
    fn rect_path() -> Result<(), image::ImageError> {
        use crate::graphs::Graph;
        use std::borrow::Cow;
        crate::logger(log::LevelFilter::Trace);
        let mut grid: Orth<RectCell> = Orth::new(4, 4);
        grid.link(0, 1).unwrap();
        grid.link(1, 5).unwrap();
        grid.link(5, 6).unwrap();
        grid.link(6, 7).unwrap();
        grid.link(7, 11).unwrap();
        grid.link(11, 10).unwrap();
        grid.link(10, 9).unwrap();
        grid.link(9, 13).unwrap();

        let mut path = path::Path::new(&grid);
        path.add(0).unwrap();
        path.add(1).unwrap();
        path.add(5).unwrap();
        path.add(6).unwrap();
        path.add(7).unwrap();
        path.add(11).unwrap();
        path.add(10).unwrap();
        path.add(9).unwrap();
        path.add(13).unwrap();
        let graph_renderer = grid.build_render().finish();
        let opts = opts::Path::default();
        let path_renderer = State {
            state: Cow::Borrowed(&graph_renderer),
            path: Cow::Borrowed(&path),
            opts: Cow::Borrowed(&opts),
        };
        path_renderer
            .render_image()
            .save("images/tests/rect_path.png")
    }
}
