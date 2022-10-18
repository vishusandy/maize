use crate::graphs::{Graph, Node};
use crate::util::AddUpdate;

#[derive(Clone, Debug)]
pub struct Dist {
    dist: Vec<Option<usize>>,
    start: usize,
    max: usize,
}

impl Dist {
    pub fn simple<G: Graph>(graph: &G, start: usize) -> Self {
        distance_simple(graph, start)
    }

    pub(crate) fn blank<G: Graph>(graph: &G) -> Self {
        Self {
            dist: vec![None; graph.len()],
            start: 0,
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

    fn start(&mut self, start: usize) {
        self.start = start;
    }

    /// Add a new value or update an existing value if it is smaller.
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

    pub fn shortest_path<G: Graph>(
        &self,
        graph: &G,
        end: usize,
    ) -> Result<crate::algo::path::Path, crate::error::Error> {
        if let Some(d) = self.dist(end) {
            crate::algo::path::Path::shortest_path(graph, self, end)
        } else {
            Err(crate::error::Error::NoPathAvailable(end))
        }
    }
}

pub(crate) fn distance_simple<G: Graph>(graph: &G, start: usize) -> Dist {
    let mut dist = Dist::blank(graph);
    let mut max = 0;
    let mut frontier: Vec<usize> = Vec::with_capacity(G::Node::N);
    frontier.push(start);
    dist.start(start);
    dist.add(start, 0);

    while !frontier.is_empty() {
        let mut frontier2: Vec<usize> = Vec::with_capacity(frontier.len() * G::Node::N);
        for cell in &frontier {
            let d = dist.dist(*cell).unwrap();
            for link in graph.node(*cell).links() {
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
    #[test]
    fn rect_dist() -> Result<(), image::ImageError> {
        crate::logger(log::LevelFilter::Trace);
        use crate::render::opts;
        use crate::render::state::dist::State;
        use std::borrow::Cow;

        let grid = crate::test::rect();

        let dist = distance_simple(&grid, 0);

        #[cfg(test)]
        log::debug!("{:#?}", dist);

        let graph_renderer = grid.build_render().finish();
        let dist_renderer = State {
            state: Cow::Borrowed(&graph_renderer),
            dist: Cow::Borrowed(&dist),
            opts: Cow::Owned(opts::DistOpts::default()),
        };
        dist_renderer.render().save("images/tests/rect_dist.png")
    }
}
