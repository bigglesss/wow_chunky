use std::path::PathBuf;

use binread::{BinReaderExt, io::Cursor};

use crate::error::Error;

pub mod macros;
pub mod types;

#[derive(Debug, Default)]
pub struct ADT {
    pub mver: Option<types::MVER>,
    pub mhdr: Option<types::MHDR>,
    pub mcin: Option<types::MCIN>,
    pub mtex: Option<types::MTEX>,
    pub mmdx: Option<types::MMDX>,
    pub mmid: Option<types::MMID>,
    pub mwmo: Option<types::MWMO>,
    pub mwid: Option<types::MWID>,
    pub mddf: Option<types::MDDF>,
    pub modf: Option<types::MODF>,
    pub mcnk: Vec<types::MCNK>,
}

pub fn parse_adt(path: PathBuf, mphd_flags: types::MPHDFlags) -> Result<ADT, Error> {
    let file = std::fs::read(path).map_err(Error::IO)?;

    let mut cursor = Cursor::new(file);
    let mut parsed_adt = ADT::default();

    loop {
        if let Some(chunk_wrapper) = cursor.read_le::<types::ChunkWrapper>().ok() {
            match chunk_wrapper.token.as_str() {
                "MVER" => macros::parse_chunk!(types::MVER, &chunk_wrapper.data, &mut parsed_adt.mver),
                "MHDR" => macros::parse_chunk!(types::MHDR, &chunk_wrapper.data, &mut parsed_adt.mhdr),
                "MCIN" => macros::parse_chunk!(types::MCIN, &chunk_wrapper.data, &mut parsed_adt.mcin),
                "MTEX" => macros::parse_chunk!(types::MTEX, &chunk_wrapper.data, &mut parsed_adt.mtex),
                "MMDX" => macros::parse_chunk!(types::MMDX, &chunk_wrapper.data, &mut parsed_adt.mmdx),
                "MMID" => macros::parse_chunk!(types::MMID, &chunk_wrapper.data, &mut parsed_adt.mmid),
                "MWMO" => macros::parse_chunk!(types::MWMO, &chunk_wrapper.data, &mut parsed_adt.mwmo),
                "MWID" => macros::parse_chunk!(types::MWID, &chunk_wrapper.data, &mut parsed_adt.mwid),
                "MDDF" => macros::parse_chunk!(types::MDDF, &chunk_wrapper.data, &mut parsed_adt.mddf),
                "MODF" => macros::parse_chunk!(types::MODF, &chunk_wrapper.data, &mut parsed_adt.modf),
                "MCNK" => {
                    let chunk = parse_chunk_data_args::<types::MCNK>(&chunk_wrapper.data, (mphd_flags.has_height_texturing, ))?;
                    parsed_adt.mcnk.push(chunk);
                },
                _ => panic!("Unknown chunk type {}!", chunk_wrapper.token),
            };
        } else {
            return Ok(parsed_adt);
        }
    }
}

#[derive(Debug, Default)]
pub struct WDT {
    pub mver: Option<types::MVER>,
    pub mphd: Option<types::MPHD>,
    pub main: Option<types::MAIN>,
    pub mwmo: Option<types::MWMO>,
    pub modf: Option<types::MODF>,
}

pub fn parse_wdt(path: PathBuf) -> Result<WDT, Error> {
    let file = std::fs::read(path).map_err(Error::IO)?;

    let mut cursor = Cursor::new(file);
    let mut parsed_wdt = WDT::default();

    loop {
        if let Some(chunk_wrapper) = cursor.read_le::<types::ChunkWrapper>().ok() {
            match chunk_wrapper.token.as_str() {
                "MVER" => macros::parse_chunk!(types::MVER, &chunk_wrapper.data, &mut parsed_wdt.mver),
                "MPHD" => macros::parse_chunk!(types::MPHD, &chunk_wrapper.data, &mut parsed_wdt.mphd),
                "MAIN" => macros::parse_chunk!(types::MAIN, &chunk_wrapper.data, &mut parsed_wdt.main),
                "MWMO" => macros::parse_chunk!(types::MWMO, &chunk_wrapper.data, &mut parsed_wdt.mwmo),
                "MODF" => macros::parse_chunk!(types::MODF, &chunk_wrapper.data, &mut parsed_wdt.modf),
                _ => panic!("Unknown chunk type {}!", chunk_wrapper.token),
            };
        } else {
            return Ok(parsed_wdt);
        }
    }
}

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