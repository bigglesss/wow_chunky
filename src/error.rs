use std::path::PathBuf;

use thiserror::Error;

/// Errors that can result from parsing the chunked files.
#[derive(Error, Debug)]
pub enum Error {
    /// Returned when a file path does not end with a valid filename.
    #[error("Invalid file name: {0:?}")]
    File(PathBuf),
    /// Wraps std::io errors.
    #[error("Error reading from file: {0}")]
    IO(#[from] std::io::Error),
    /// Wraps BinRead errors.
    #[error("Unknown parsing error: {0}")]
    Unknown(#[from] binread::Error),
}
