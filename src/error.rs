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
}
