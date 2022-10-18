use crate::graphs::{Graph, Node};

#[derive(Clone, Debug)]
pub struct Path {
    /// Ordered listing of visited cell ids
    pub(crate) path: Vec<usize>,
    /// Reverse lookup to find what step number a given cell id has
    cells: Vec<Option<usize>>,
    /// The path length to use when calculating intentsity.
    /// A None value uses the path length while a Some() value instead uses a specified length.
    pub(crate) max: Option<usize>,
}
impl Path {
    pub fn blank<G: Graph>(graph: &G) -> Self {
        Self {
            /// maps steps (index) to cell ids (values)
            path: Vec::with_capacity(graph.len() / G::Node::N),
            /// tracks which step number each cell has
            cells: vec![None; graph.len()],
            max: None,
        }
    }

    pub(crate) fn with_capacity<G: Graph>(graph: &G, max: usize) -> Self {
        Self {
            path: Vec::with_capacity(max + 1),
            cells: vec![None; graph.len()],
            max: Some(max),
        }
    }

    pub fn shortest_path<G: Graph>(
        graph: &G,
        dist: &crate::algo::dist::Dist,
        end: usize,
    ) -> Result<Self, crate::Error> {
        shortest_path(graph, dist, end)
    }

    pub fn add(&mut self, id: usize) -> Result<(), crate::Error> {
        if self.cells[id].is_none() {
            self.cells[id] = Some(self.path.len());
            self.path.push(id);
            Ok(())
        } else {
            Err(crate::Error::InvalidPathAdd(id))
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

    pub(crate) fn reverse(&mut self) {
        self.path.reverse();
        let max = self.max.unwrap_or(self.path.len() - 1);
        self.cells
            .iter_mut()
            .flatten()
            .for_each(|step| *step = max - *step);
    }
}

fn shortest_path<G: Graph>(
    graph: &G,
    dist: &crate::algo::dist::Dist,
    end: usize,
) -> Result<crate::algo::path::Path, crate::error::Error> {
    if let Some(d) = dist.dist(end) {
        let mut path = Path::with_capacity(graph, d);
        let mut cell = graph.node(end);

        for _ in 0..=d {
            path.add(cell.id())?;

            let prev = cell
                .links()
                .reduce(|min, id| {
                    if dist.dist(*id) < dist.dist(*min) {
                        id
                    } else {
                        min
                    }
                })
                .unwrap();
            cell = graph.node(*prev);
        }
        path.reverse();
        Ok(path)
    } else {
        Err(crate::error::Error::NoPathAvailable(end))
    }
}
