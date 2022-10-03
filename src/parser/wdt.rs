use std::fs::File;
use std::path::PathBuf;

use binread::BinReaderExt;

use crate::error::Error;
use crate::parser::macros;
use crate::types::chunks;

use super::parse_chunk_data;

#[derive(Debug, Default)]
pub struct WDT {
    pub key: String,

    pub mver: Option<chunks::MVER>,
    pub mphd: Option<chunks::MPHD>,
    pub main: Option<chunks::MAIN>,
    pub mwmo: Option<chunks::MWMO>,
    pub modf: Option<chunks::MODF>,
}

pub fn parse_wdt_file(key: String, mut file: File) -> Result<WDT, Error> {
    let mut parsed_wdt = WDT {
        key,
        ..Default::default()
    };

    loop {
        if let Some(chunk_wrapper) = file.read_le::<chunks::shared::ChunkWrapper>().ok() {
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

impl WDT {
    pub fn from_file(path: &PathBuf) -> Result<Self, Error> {
        let file = File::open(&path)?;

        let key = path.file_name().ok_or(Error::File(path.clone()))?
            .to_string_lossy().to_string();

        Ok(parse_wdt_file(key, file)?) 
    }
}