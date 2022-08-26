pub(crate) mod orth;

pub(crate) trait Block {}

pub(crate) trait Node {
    type Block: Block;
    const N: usize;
    fn id(&self) -> usize;
    fn all_neighbors(&self) -> &[Option<usize>];
    fn neighbors(&self) -> Neighbors<Self>
    where
        Self: Sized;
    fn num_neighbors(&self) -> usize;
    fn neighbor(&self, n: usize) -> Option<usize>;
    fn neighbor_id(&self, cell: usize) -> Option<usize>;
}

pub(crate) trait Graph {
    type Node: Node;
    fn cell(&self, id: usize) -> Self::Node;
    fn len(&self) -> usize;
    fn cell_mut(&mut self, id: usize) -> &mut Self::Node;
    fn cells(&self) -> Box<dyn Iterator<Item = &Self::Node> + '_>;
}

// https://adventures.michaelfbryan.com/posts/daily/iterators/
// https://aloso.github.io/2021/03/09/creating-an-iterator
pub(crate) struct Neighbors<'a, N: Node> {
    slice: &'a [Option<usize>],
    _phantom: std::marker::PhantomData<N>,
}
impl<'a, N: Node> Iterator for Neighbors<'a, N> {
    type Item = &'a usize;
    fn next(&mut self) -> Option<Self::Item> {
        match self.slice {
            [first, rest @ ..] => {
                self.slice = rest;
                if let Some(f) = first {
                    Some(f)
                } else {
                    self.next()
                }
            }
            [] => None,
        }
    }
}
