use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid operation: no neighbor from node {0} to {1}")]
    InvalidNeighbor(usize, usize),
    #[error("Already linked: cell {0} is already linked with {1}")]
    AlreadyLinked(usize, usize),
    #[error("Unlink operation failed: cell {0} is not linked with {1}")]
    AlreadyUnlinked(usize, usize),
    #[error("Invalid cell: id {0} was specified but grid only contains {1} cells")]
    InvalidCell(usize, usize),
    #[error("Invalid id: graph contains only {1} nodes and node {0} was specified")]
    InvalidId(usize, usize),
    #[error("Invalid neighbor: neighbor id {0} was specified but the node only has {1} neighbors")]
    InvalidNeighborId(usize, usize),
    #[error("Invalid edge: id {0} with neighbor id {1} was not found")]
    InvalidEdge(usize, usize),
    #[error("Mismatched list size: list has a length of {0} while graph has a length of {1}")]
    MismatchedListSize(usize, usize),
}
