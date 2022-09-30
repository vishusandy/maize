use crate::graphs::{Graph, Node};
use crate::DEFAULT_NEIGHBORS;
use smallvec::SmallVec;

#[derive(Clone, Debug)]
pub(crate) struct UndirEdge<V> {
    a: Conn,
    b: Conn,
    v: V,
}

impl<V> UndirEdge<V> {
    fn new(a: Conn, b: Conn, v: V) -> Self {
        Self { a, b, v }
    }

    fn map<G: Graph, U>(&self, grid: &G, value: U) -> UndirEdge<U> {
        UndirEdge {
            a: self.a.clone(),
            b: self.b.clone(),
            v: value,
        }
    }

    fn map_with<G: Graph, F: Fn(&G, &Conn) -> U, U>(&self, grid: &G, f: F) -> UndirEdge<U> {
        UndirEdge {
            a: self.a.clone(),
            b: self.b.clone(),
            v: f(grid, &self.a),
        }
    }

    pub(crate) fn a(&self) -> &Conn {
        &self.a
    }

    pub(crate) fn b(&self) -> &Conn {
        &self.b
    }

    pub(crate) fn cells(&self) -> (&Conn, &Conn) {
        (&self.a, &self.b)
    }

    pub(crate) fn value(&self) -> &V {
        &self.v
    }

    pub(crate) fn has_id(&self, id: usize) -> bool {
        (self.a.id == id) | (self.b.id == id)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Conn {
    id: usize,
    side: usize,
}

impl Conn {
    fn new(id: usize, side: usize) -> Self {
        Self { id, side }
    }
    pub(crate) fn id(&self) -> usize {
        self.id
    }
    pub(crate) fn side(&self) -> usize {
        self.side
    }
}

impl From<(usize, usize)> for Conn {
    fn from(a: (usize, usize)) -> Self {
        Self { id: a.0, side: a.1 }
    }
}

#[derive(Clone, Debug)]
pub struct Undirected<V> {
    cells: Vec<SmallVec<[Option<usize>; DEFAULT_NEIGHBORS]>>,
    edges: Vec<UndirEdge<V>>, // edge id is the index
    outside: Vec<(Conn, V)>,
}

impl<V> Undirected<V> {
    pub(crate) fn new<G: Graph>(grid: &G, default: V) -> Self
    where
        V: Copy,
    {
        let f = |_: &G, _: usize| default;
        Self::new_with(grid, f, f)
    }
    pub(crate) fn new_with<G: Graph, F: Fn(&G, usize) -> V>(grid: &G, inner: F, outer: F) -> Self {
        let mut cells: Vec<SmallVec<[Option<usize>; DEFAULT_NEIGHBORS]>> = (0..grid.len())
            .map(|i| (0..grid.cell(i).max_neighbors()).map(|_| None).collect())
            .collect();
        let mut edges: Vec<UndirEdge<V>> = Vec::with_capacity(grid.len() * G::Node::N); // allocate enough space to accomodate all edges - even in a wrap-around style grid (e.g., a cyclinder)
        let mut outside: Vec<(Conn, V)> = Vec::with_capacity(grid.len()); // default capacity could be improved here with a Graph method
        for cell in grid.cells() {
            for (i, neighbor) in cell.all_neighbors().iter().enumerate() {
                if let Some(n) = neighbor {
                    let neighbor = grid.cell(*n);
                    if let Some(nside) = neighbor.neighbor_id(cell.id()) {
                        if let Some(e) = cells[*n][nside] {
                            cells[cell.id()][i] = Some(e); // an existing neighbor has listed this edge already - use the corresponding edge
                            continue;
                        }
                        // no existing edge found - create a new one
                        cells[cell.id()][i] = Some(edges.len());
                        let a = Conn::new(cell.id(), i);
                        let b = Conn::new(*n, nside);
                        let edge: UndirEdge<V> = UndirEdge::new(a, b, inner(grid, i));
                        edges.push(edge);
                        continue;
                    }
                }
                #[cfg(test)]
                log::debug!("Adding outer edge: id={} n={}", cell.id(), i);
                outside.push((Conn::new(cell.id(), i), outer(grid, i))); // the current edge is an outer edge - add it to the list
            }
        }
        Self {
            cells,
            edges,
            outside,
        }
    }

    pub(crate) fn iter(&self) -> std::slice::Iter<UndirEdge<V>> {
        self.edges.iter()
    }

    pub(crate) fn iter_outer(&self) -> std::slice::Iter<(Conn, V)> {
        self.outside.iter()
    }

    pub(crate) fn edges(&self) -> &Vec<UndirEdge<V>> {
        &self.edges
    }

    pub(crate) fn outer(&self) -> &Vec<(Conn, V)> {
        &self.outside
    }

    pub(crate) fn map<G: Graph, U>(&self, grid: &G, inner: &U, outer: &U) -> Undirected<U>
    where
        U: Clone,
    {
        Undirected {
            cells: self.cells.clone(),
            edges: self
                .edges
                .iter()
                .map(|e| e.map(grid, inner.clone()))
                .collect(),
            outside: {
                let mut v: Vec<(Conn, U)> = Vec::with_capacity(self.outside.len());
                self.outside
                    .iter()
                    .for_each(|x| v.push((x.0.clone(), outer.clone())));
                v
            },
        }
    }

    /// The map method makes a new `Undirected` edge list with a different stored value.  This avoids the overhead of mapping each cell edges to borders.
    pub(crate) fn map_with<G: Graph, F: Fn(&G, &Conn) -> U, U>(
        &self,
        grid: &G,
        inner: F,
        outer: F,
    ) -> Undirected<U> {
        Undirected {
            cells: self.cells.clone(),
            edges: self
                .edges
                .iter()
                .map(|e| e.map_with(grid, &inner))
                .collect(),
            outside: {
                let mut v: Vec<(Conn, U)> = Vec::with_capacity(self.outside.len());
                self.outside
                    .iter()
                    .for_each(|x| v.push((x.0.clone(), outer(grid, &x.0))));
                v
            },
        }
    }
}
