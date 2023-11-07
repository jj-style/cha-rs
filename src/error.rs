use thiserror::Error;

/// ExtractError represents an error that can occur when extracting characters from an input.
#[derive(Error, Debug)]
pub enum ExtractError {
    #[error("must provide at least one index to extract")]
    NoIndices,
    #[error("all indices must be within range of input: 1-{0}")]
    OutOfRange(usize),
    #[error("unknown error")]
    Unknown,
}