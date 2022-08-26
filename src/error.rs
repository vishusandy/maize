use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid operation: no neighbor from node {0} to {1}")]
    InvalidNeighbor(usize, usize),
}
