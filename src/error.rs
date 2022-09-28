use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error reading from file: {0}")]
    IO(#[from] std::io::Error),
    #[error("Unknown parsing error: {0}")]
    Unknown(#[from] binread::Error),
}
