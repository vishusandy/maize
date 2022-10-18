pub(crate) mod orth;
pub(crate) mod weight;

use smallvec::SmallVec;

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
    /// Returns the neighbor id of the shared edge between self and the specified cell
    fn neighbor_id(&self, cell: usize) -> Option<usize>;
    fn links(&self) -> Neighbors;
    fn linked_to(&self, id: usize) -> bool;
    fn linked_side(&self, n: usize) -> bool;
    fn num_links(&self) -> usize {
        self.links().count()
    }
    /// whether the node has any links or not
    fn is_empty(&self) -> bool {
        self.num_links() == 0
    }
    fn link(&mut self, cell: usize) -> Result<(), crate::Error>;
    fn unlink(&mut self, cell: usize) -> Result<(), crate::Error>;
}

pub trait Graph {
    type Node: Node;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn len(&self) -> usize;
    fn node(&self, id: usize) -> &Self::Node;
    fn node_mut(&mut self, id: usize) -> &mut Self::Node;
    fn node_ids(&self) -> Vec<usize> {
        self.nodes().map(|n| n.id()).collect()
    }
    /// This should iterate over all cells in id order
    fn nodes(&self) -> Box<dyn Iterator<Item = &Self::Node> + '_>;
    fn nodes_mut(&mut self) -> Box<dyn Iterator<Item = &mut Self::Node> + '_>;
    fn link(&mut self, a: usize, b: usize) -> Result<(), crate::Error>;
    fn unlink(&mut self, a: usize, b: usize) -> Result<(), crate::Error>;

    fn random<R>(&self, rng: &mut R) -> Option<usize>
    where
        R: rand::Rng + ?Sized,
    {
        use rand::seq::SliceRandom;
        self.node_ids().choose(rng).copied()
    }

    fn linked_neighbors(&self, node: usize) -> SmallVec<[usize; crate::DEFAULT_NEIGHBORS]> {
        self.node(node)
            .neighbors()
            .map(|n| self.node(*n))
            .filter(|n| !n.is_empty())
            .map(|n| n.id())
            .collect()
    }

    fn unlinked_neighbors(&self, node: usize) -> SmallVec<[usize; crate::DEFAULT_NEIGHBORS]> {
        self.node(node)
            .neighbors()
            .map(|n| self.node(*n))
            .filter(|n| n.is_empty())
            .map(|n| n.id())
            .collect()
    }
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
                first.as_ref().map(Some).unwrap_or_else(|| self.next())
            }
            [] => None,
        }
    }
}
