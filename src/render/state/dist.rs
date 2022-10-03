use super::RenderGraph;
use crate::algo::dist::Dist;
use crate::graphs::{Graph, Node};
use crate::render::opts;
use crate::render::RenderState;
use image::RgbaImage;

#[derive(Clone, Debug)]
pub struct State<'b, 'c, 'e, 'g, 'o, 'p, 'po, 'r, G>
where
    G: RenderGraph + Clone + std::fmt::Debug,
    <G::Node as Node>::Block: Clone + std::fmt::Debug,
{
    pub(crate) state:
        std::borrow::Cow<'r, crate::render::state::graph::State<'b, 'c, 'e, 'g, 'o, G>>,
    pub(crate) dist: std::borrow::Cow<'p, Dist>,
    pub(crate) opts: std::borrow::Cow<'po, opts::Dist>,
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
        self.edges(&mut image);
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

    fn edges(&self, image: &mut RgbaImage) {
        self.state.edges(image)
    }
}
