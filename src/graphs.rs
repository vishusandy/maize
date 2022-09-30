pub(crate) mod orth;

pub trait Block {}

pub trait Node {
    type Block: Block;
    const N: usize;
    fn id(&self) -> usize;
    fn all_neighbors(&self) -> &[Option<usize>];
    fn neighbors(&self) -> Neighbors;
    fn num_neighbors(&self) -> usize;
    fn max_neighbors(&self) -> usize {
        Self::N
    }
    fn neighbor(&self, n: usize) -> Option<usize>;
    fn neighbor_id(&self, cell: usize) -> Option<usize>;
    fn links(&self) -> Neighbors;
    fn linked_to(&self, id: usize) -> bool;
    fn linked_side(&self, n: usize) -> bool;
    fn link(&mut self, cell: usize) -> Result<(), crate::error::Error>;
    fn unlink(&mut self, cell: usize) -> Result<(), crate::error::Error>;
}

pub trait Graph {
    type Node: Node;
    fn len(&self) -> usize;
    fn cell(&self, id: usize) -> &Self::Node;
    fn cell_mut(&mut self, id: usize) -> &mut Self::Node;
    fn cells(&self) -> Box<dyn Iterator<Item = &Self::Node> + '_>;
    fn link(&mut self, a: usize, b: usize) -> Result<(), crate::error::Error>;
    fn unlink(&mut self, a: usize, b: usize) -> Result<(), crate::error::Error>;
}

// https://adventures.michaelfbryan.com/posts/daily/iterators/
// https://aloso.github.io/2021/03/09/creating-an-iterator
pub struct Neighbors<'a> {
    slice: &'a [Option<usize>],
}

impl<'a> Neighbors<'a> {
    pub(crate) fn new(slice: &'a [Option<usize>]) -> Self {
        Self { slice }
    }
}

impl<'a> Iterator for Neighbors<'a> {
    type Item = &'a usize;
    fn next(&mut self) -> Option<Self::Item> {
        match self.slice {
            [first, rest @ ..] => {
                self.slice = rest;
                first
                    .as_ref()
                    .map(|s| Some(s))
                    .unwrap_or_else(|| self.next())
            }
            [] => None,
        }
    }
}
