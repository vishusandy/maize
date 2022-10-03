use crate::graphs::Graph;
use crate::util::AddUpdate;

#[derive(Clone, Debug)]
pub struct Dist {
    dist: Vec<Option<usize>>,
    max: usize,
}

impl Dist {
    pub(crate) fn blank<G: Graph>(graph: &G) -> Self {
        Self {
            dist: vec![None; graph.len()],
            max: 0,
        }
    }

    pub fn dist(&self, id: usize) -> Option<usize> {
        if id >= self.dist.len() {
            return None;
        }
        self.dist[id]
    }

    pub fn max(&self) -> usize {
        self.max
    }

    pub fn set_max(&mut self, max: usize) {
        self.max = max;
    }

    /// Add a new value or update if the cell exists but has a greater value.
    fn add(&mut self, id: usize, dist: usize) -> AddUpdate {
        match self.dist(id) {
            Some(d) => {
                if dist < d {
                    self.dist[id] = Some(dist);
                    return AddUpdate::Updated;
                }
                AddUpdate::Exists
            }
            None => {
                self.dist[id] = Some(dist);
                AddUpdate::Added
            }
        }
    }
}

pub(crate) fn distance_simple<G: Graph>(graph: &G, start: usize) -> Dist {
    use crate::graphs::Node;
    let mut dist = Dist::blank(graph);
    let mut max = 0;
    let mut frontier: Vec<usize> = Vec::with_capacity(G::Node::N);
    frontier.push(start);
    dist.add(start, 0);

    while !frontier.is_empty() {
        let mut frontier2: Vec<usize> = Vec::with_capacity(frontier.len() * G::Node::N);
        for cell in &frontier {
            let d = dist.dist(*cell).unwrap();
            for link in graph.cell(*cell).links() {
                if dist.add(*link, d + 1).added() {
                    frontier2.push(*link);
                    if d + 1 > max {
                        max = d + 1;
                    }
                }
            }
        }
        frontier = frontier2;
    }

    dist.set_max(max);
    dist
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Orth, RectCell};
    #[test]
    fn rect_dist() -> Result<(), image::ImageError> {
        crate::logger(log::LevelFilter::Trace);
        use crate::graphs::Graph;
        use crate::render::opts;
        use crate::render::state::dist::State;
        use std::borrow::Cow;

        let mut grid: Orth<RectCell> = Orth::new(4, 4);

        grid.link(0, 1).unwrap();
        grid.link(0, 4).unwrap();
        grid.link(4, 8).unwrap();
        grid.link(8, 12).unwrap();
        grid.link(12, 13).unwrap();
        grid.link(1, 5).unwrap();
        grid.link(5, 6).unwrap();
        grid.link(6, 10).unwrap();
        grid.link(10, 14).unwrap();
        grid.link(14, 15).unwrap();

        let dist = distance_simple(&grid, 0);

        #[cfg(test)]
        log::debug!("{:#?}", dist);

        let graph_renderer = grid.build_render().finish();
        let dist_renderer = State {
            state: Cow::Borrowed(&graph_renderer),
            dist: Cow::Borrowed(&dist),
            opts: Cow::Owned(opts::Dist::default()),
        };
        dist_renderer.render().save("images/tests/rect_dist.png")
    }
}
