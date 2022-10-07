//! Parsing logic and base structs for all chunked file formats.
use std::path::PathBuf;

use binread::{BinReaderExt, io::Cursor};

use crate::error::Error;

mod macros;
mod adt;
mod wdt;
mod blp;
mod bls;

pub use adt::ADT;
pub use wdt::WDT;
pub use blp::BLP;
pub use bls::BLS;

fn parse_chunk_data<T: binread::BinRead>(chunk_data: &Vec<u8>) -> Result<T, Error> {
    let mut chunk_data_cursor = Cursor::new(chunk_data);
    let chunk_data: T = chunk_data_cursor.read_le().map_err(Error::Unknown)?;

    Ok(chunk_data)
}

fn parse_chunk_data_args<T: binread::BinRead>(chunk_data: &Vec<u8>, args: T::Args) -> Result<T, Error> {
    let mut chunk_data_cursor = Cursor::new(chunk_data);
    let chunk_data: T = chunk_data_cursor.read_le_args(args).map_err(Error::Unknown)?;

    Ok(chunk_data)
}

pub fn parse_bls(path: PathBuf) -> Result<bls::BLS, Error> {
    let file = std::fs::read(path).map_err(Error::IO)?;

    let mut cursor = Cursor::new(file);

    let parsed_bls: bls::BLS = cursor.read_le()?;
    return Ok(parsed_bls);
}
