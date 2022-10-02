use super::RenderGraph;
use crate::graphs::{Graph, Node};
use crate::render::opts;
use crate::render::RenderState;
use image::{Rgba, RgbaImage};

#[derive(Clone, Debug)]
pub struct Path {
    /// Ordered listing of visited cell ids
    path: Vec<usize>,
    /// Reverse lookup to find what step number a given cell id has
    cells: Vec<Option<usize>>,
    /// The path length to use when calculating intentsity.
    /// A None value uses the path length while a Some() value instead uses a specified length.
    max: Option<usize>,
}
impl Path {
    pub fn new<G: Graph>(graph: &G) -> Self {
        Self {
            path: Vec::with_capacity(graph.len() / G::Node::N),
            cells: vec![None; graph.len()],
            max: None,
        }
    }
    pub fn add(&mut self, id: usize) -> Result<(), ()> {
        if self.cells[id].is_none() {
            self.cells[id] = Some(self.path.len());
            self.path.push(id);
            Ok(())
        } else {
            Err(())
        }
    }
    pub fn step_num(&self, cell_id: usize) -> Option<usize> {
        self.cells[cell_id]
    }
    pub fn step(&self, step: usize) -> usize {
        self.path[step]
    }
    pub fn set_max(&mut self, max: Option<usize>) {
        self.max = max;
    }
}

#[derive(Clone, Debug)]
pub struct State<'b, 'c, 'e, 'g, 'o, 'p, 'po, 'r, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
    state: std::borrow::Cow<'r, crate::render::state::graph::State<'b, 'c, 'e, 'g, 'o, G>>,
    path: std::borrow::Cow<'p, Path>,
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
        let size = self.state.opts.size();
        let (x, y) = self
            .state
            .graph
            .size(size.block_height(), size.block_width(), size.padding());
        #[allow(unused_mut)]
        let mut img = RgbaImage::from_pixel(x, y, self.state.opts.colors().maze_bg());
        for cell in self.state.graph.cells() {
            self.fill(cell, &mut img);
            if let Some(arrow) = self.opts.arrows() {
                self.arrows(cell, self.opts.style(), arrow, &mut img);
            }
            if self.state.opts.text().show() {
                if let Some(step) = self.path.step_num(cell.id()) {
                    let text = if self.opts.label_steps() {
                        step.to_string()
                    } else {
                        cell.id().to_string()
                    };
                    self.text(cell, &text, &mut img);
                }
            }
        }
        self.edges(&mut img);
        img
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

        let mut path = Path::new(&grid);
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
