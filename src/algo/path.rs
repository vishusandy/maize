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
