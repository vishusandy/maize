use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Zero sized graph: there are no nodes in this graph")]
    ZeroSizedGraph(),
    #[error("Invalid operation: no neighbor from node {0} to {1}")]
    InvalidNeighbor(usize, usize),
    #[error("Already linked: node {0} is already linked with {1}")]
    AlreadyLinked(usize, usize),
    #[error("Unlink operation failed: node {0} is not linked with {1}")]
    AlreadyUnlinked(usize, usize),
    #[error("Invalid cell: id {0} was specified but grid only contains {1} nodes")]
    InvalidCell(usize, usize),
    #[error("Invalid id: graph contains only {1} nodes and node {0} was specified")]
    InvalidId(usize, usize),
    #[error("Invalid neighbor: neighbor id {0} was specified but the node only has {1} neighbors")]
    InvalidNeighborId(usize, usize),
    #[error("Invalid edge: id {0} with neighbor id {1} was not found")]
    InvalidEdge(usize, usize),
    #[error("Mismatched list size: list has a length of {0} while graph has a length of {1}")]
    MismatchedListSize(usize, usize),
    #[error("No path available for node {0}")]
    NoPathAvailable(usize),
    #[error("Invalid path add: path already contains node {0}")]
    InvalidPathAdd(usize),
    #[error("Invalid HSL value: h must be in the range [0, 360) but found {0}")]
    InvalidHslH(f64),
    #[error("Invalid HSL value: s must be in the range [0, 1] but found {0}")]
    InvalidHslS(f64),
    #[error("Invalid HSL value: l must be in the range [0, 1] but found {0}")]
    InvalidHslL(f64),
    #[error("Animation error: {0}")]
    AnimationError(webp_animation::Error),
}

impl From<webp_animation::Error> for Error {
    fn from(error: webp_animation::Error) -> Self {
        Self::AnimationError(error)
    }
}
