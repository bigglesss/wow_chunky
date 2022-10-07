use std::path::PathBuf;

use thiserror::Error;

/// Errors that can result from parsing the chunked files.
#[derive(Error, Debug)]
pub enum Error {
    /// Returned when a file does not have with a valid filename for its type.
    #[error("Invalid file name: {0:?}")]
    InvalidFilename(PathBuf),
    /// Returned when an ADT does not have X/Y coordinates in the filename.
    #[error("Invalid ADT, missing X/Y coordinates: {0:?}")]
    MissingCoordinates(PathBuf),
    /// Returned when a file does not exist.
    #[error("No file found at: {0:?}")]
    FileNotFound(PathBuf),
    /// Wraps std::io errors.
    #[error("Error reading from file: {0}")]
    IO(#[from] std::io::Error),
    /// Wraps BinRead errors.
    #[error("Unknown parsing error: {0}")]
    Unknown(#[from] binread::Error),
}
