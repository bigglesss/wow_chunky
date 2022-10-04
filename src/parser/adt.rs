use std::fs::File;
use std::path::PathBuf;

use binread::BinReaderExt;

use crate::error::Error;
use crate::parser::{macros, wdt};
use crate::types::chunks;
use crate::types::shared;

use super::{parse_chunk_data,parse_chunk_data_args};

#[derive(Clone, Debug, Default)]
pub struct ADT {
    pub filename: String,
    pub path: PathBuf,

    pub mver: Option<shared::MVER>,
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

fn parse_adt_file(path: PathBuf, mphd_flags: &chunks::MPHDFlags) -> Result<ADT, Error> {
    let filename = path.file_name().ok_or(Error::File(path.clone()))?
        .to_string_lossy().to_string();

    let mut file = File::open(&path)?;

    let mut parsed_adt = ADT {
        filename,
        path,
        ..Default::default()
    };

    loop {
        if let Some(chunk_wrapper) = file.read_le::<shared::ChunkWrapper>().ok() {
            match chunk_wrapper.token.as_str() {
                "MVER" => macros::parse_chunk!(shared::MVER, &chunk_wrapper.data, &mut parsed_adt.mver),
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

impl ADT {
    pub fn from_file(path: PathBuf, mphd_flags: &chunks::MPHDFlags) -> Result<Self, Error> {
        Ok(parse_adt_file(path, mphd_flags)?) 
    }

    pub fn from_wdt_file(wdt_filename: PathBuf, x: u32, y: u32) -> Result<Self, Error> {
        let adt_name = format!("{}_{}_{}.adt", &wdt_filename.file_stem().and_then(|n| n.to_str()).expect("WDT should have a extension."), x, y);
        let adt_path = wdt_filename
            .parent().expect("WDT file should be in a folder with the ADT files.")
            .join(adt_name);

        let wdt = wdt::WDT::from_file(wdt_filename)?;
        let adt = ADT::from_file(adt_path, &wdt.mphd.and_then(|chunk| Some(chunk.flags)).expect("WDT should have a valid MPHD chunk"))?;

        Ok(adt)
    }
    
    pub fn from_wdt(wdt: &wdt::WDT, x: u32, y: u32) -> Result<Self, Error> {
        let adt_name = format!("{}_{}_{}.adt", wdt.path.file_stem().and_then(|n| n.to_str()).expect("WDT should have a extension."), x, y);
        let adt_path = wdt.path
            .parent().expect("WDT file should be in a folder with the ADT files.")
            .join(adt_name);

        let adt = ADT::from_file(adt_path, &wdt.mphd.as_ref().and_then(|chunk| Some(&chunk.flags)).expect("WDT should have a valid MPHD chunk"))?;

        Ok(adt) 
    }
}
