use super::Graph;
use crate::edges;
use crate::Error;

#[derive(Clone, Debug)]
pub struct Uniform {
    weights: edges::Undirected<usize>,
}
impl Uniform {
    pub fn from_slice<const N: usize, G: Graph>(
        graph: &G,
        slice: &[[usize; N]],
    ) -> Result<Self, Error> {
        if slice.len() != graph.len() {
            Err(Error::MismatchedListSize(slice.len(), graph.len()))
        } else {
            Ok(Self {
                weights: edges::Undirected::new_with(graph, |g, id, n| slice[id][n], |g, id, n| 0),
            })
        }
    }
    pub fn weight(&self, id: usize, n: usize) -> Result<usize, Error> {
        self.weights.edge_value(id, n).copied()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;
    use rand::SeedableRng;
    use rand_xoshiro::SplitMix64;
    #[test]
    fn uniform_weights() {
        use crate::Orth;
        use crate::RectCell;
        crate::logger(log::LevelFilter::Trace);

        let rng = &mut SplitMix64::seed_from_u64(13131313131313131313);
        let grid: Orth<RectCell> = Orth::new(4, 4);
        let mut w = [[0; 4]; 16];
        for id in w.iter_mut() {
            for n in id.iter_mut() {
                *n = rng.gen_range(1..=40);
            }
        }
        let weights = Uniform::from_slice(&grid, &w).unwrap();
        log::debug!("{:?}", weights);
    }
}
