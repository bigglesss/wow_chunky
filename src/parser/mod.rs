use std::path::PathBuf;

use binread::{BinReaderExt, io::Cursor};

use crate::error::Error;
use crate::types::chunks;

mod macros;

#[derive(Debug, Default)]
pub struct ADT {
    pub mver: Option<chunks::MVER>,
    pub mhdr: Option<chunks::MHDR>,
    pub mcin: Option<chunks::MCIN>,
    pub mtex: Option<chunks::MTEX>,
    pub mmdx: Option<chunks::MMDX>,
    pub mmid: Option<chunks::MMID>,
    pub mwmo: Option<chunks::MWMO>,
    pub mwid: Option<chunks::MWID>,
    pub mddf: Option<chunks::MDDF>,
    pub modf: Option<chunks::MODF>,
    pub mcnk: Vec<chunks::MCNK>,
}

pub fn parse_adt(path: PathBuf, mphd_flags: chunks::MPHDFlags) -> Result<ADT, Error> {
    let file = std::fs::read(path).map_err(Error::IO)?;

    let mut cursor = Cursor::new(file);
    let mut parsed_adt = ADT::default();

    loop {
        if let Some(chunk_wrapper) = cursor.read_le::<chunks::shared::ChunkWrapper>().ok() {
            match chunk_wrapper.token.as_str() {
                "MVER" => macros::parse_chunk!(chunks::MVER, &chunk_wrapper.data, &mut parsed_adt.mver),
                "MHDR" => macros::parse_chunk!(chunks::MHDR, &chunk_wrapper.data, &mut parsed_adt.mhdr),
                "MCIN" => macros::parse_chunk!(chunks::MCIN, &chunk_wrapper.data, &mut parsed_adt.mcin),
                "MTEX" => macros::parse_chunk!(chunks::MTEX, &chunk_wrapper.data, &mut parsed_adt.mtex),
                "MMDX" => macros::parse_chunk!(chunks::MMDX, &chunk_wrapper.data, &mut parsed_adt.mmdx),
                "MMID" => macros::parse_chunk!(chunks::MMID, &chunk_wrapper.data, &mut parsed_adt.mmid),
                "MWMO" => macros::parse_chunk!(chunks::MWMO, &chunk_wrapper.data, &mut parsed_adt.mwmo),
                "MWID" => macros::parse_chunk!(chunks::MWID, &chunk_wrapper.data, &mut parsed_adt.mwid),
                "MDDF" => macros::parse_chunk!(chunks::MDDF, &chunk_wrapper.data, &mut parsed_adt.mddf),
                "MODF" => macros::parse_chunk!(chunks::MODF, &chunk_wrapper.data, &mut parsed_adt.modf),
                "MCNK" => {
                    let chunk = parse_chunk_data_args::<chunks::MCNK>(&chunk_wrapper.data, (mphd_flags.has_height_texturing, ))?;
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
    pub mver: Option<chunks::MVER>,
    pub mphd: Option<chunks::MPHD>,
    pub main: Option<chunks::MAIN>,
    pub mwmo: Option<chunks::MWMO>,
    pub modf: Option<chunks::MODF>,
}

pub fn parse_wdt(path: PathBuf) -> Result<WDT, Error> {
    let file = std::fs::read(path).map_err(Error::IO)?;

    let mut cursor = Cursor::new(file);
    let mut parsed_wdt = WDT::default();

    loop {
        if let Some(chunk_wrapper) = cursor.read_le::<chunks::shared::ChunkWrapper>().ok() {
            match chunk_wrapper.token.as_str() {
                "MVER" => macros::parse_chunk!(chunks::MVER, &chunk_wrapper.data, &mut parsed_wdt.mver),
                "MPHD" => macros::parse_chunk!(chunks::MPHD, &chunk_wrapper.data, &mut parsed_wdt.mphd),
                "MAIN" => macros::parse_chunk!(chunks::MAIN, &chunk_wrapper.data, &mut parsed_wdt.main),
                "MWMO" => macros::parse_chunk!(chunks::MWMO, &chunk_wrapper.data, &mut parsed_wdt.mwmo),
                "MODF" => macros::parse_chunk!(chunks::MODF, &chunk_wrapper.data, &mut parsed_wdt.modf),
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
