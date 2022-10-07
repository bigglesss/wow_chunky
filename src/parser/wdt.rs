use std::fs::File;
use std::path::PathBuf;

use binread::BinReaderExt;

use crate::error::Error;
use crate::parser::macros;

use crate::types;

use super::parse_chunk_data;

#[derive(Clone, Debug, Default)]
pub struct WDT {
    pub filename: String,
    pub path: PathBuf,

    pub mver: Option<types::shared::MVER>,
    pub mphd: Option<types::wdt::MPHD>,
    pub main: Option<types::wdt::MAIN>,
    pub mwmo: Option<types::shared::MWMO>,
    pub modf: Option<types::shared::MODF>,
}

pub fn parse_wdt_file(path: PathBuf) -> Result<WDT, Error> {
    let filename = path.file_name().ok_or(Error::File(path.clone()))?
        .to_string_lossy().to_string();

    let mut file = File::open(&path)?;

    let mut parsed_wdt = WDT {
        filename,
        path,
        ..Default::default()
    };

    loop {
        if let Some(chunk_wrapper) = file.read_le::<types::shared::ChunkWrapper>().ok() {
            match chunk_wrapper.token.as_str() {
                "MVER" => macros::parse_chunk!(types::shared::MVER, &chunk_wrapper.data, &mut parsed_wdt.mver),
                "MPHD" => macros::parse_chunk!(types::wdt::MPHD, &chunk_wrapper.data, &mut parsed_wdt.mphd),
                "MAIN" => macros::parse_chunk!(types::wdt::MAIN, &chunk_wrapper.data, &mut parsed_wdt.main),
                "MWMO" => macros::parse_chunk!(types::shared::MWMO, &chunk_wrapper.data, &mut parsed_wdt.mwmo),
                "MODF" => macros::parse_chunk!(types::shared::MODF, &chunk_wrapper.data, &mut parsed_wdt.modf),
                _ => panic!("Unknown chunk type {}!", chunk_wrapper.token),
            };
        } else {
            return Ok(parsed_wdt);
        }
    }
}

impl WDT {
    pub fn from_file(path: PathBuf) -> Result<Self, Error> {
        Ok(parse_wdt_file(path)?) 
    }
}
