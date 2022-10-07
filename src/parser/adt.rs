use std::fs::File;
use std::path::PathBuf;

use binread::BinReaderExt;

use crate::error::Error;

use crate::parser::macros;
use crate::parser::wdt;

use crate::types;

use super::{parse_chunk_data, parse_chunk_data_args};

#[derive(Clone, Debug, Default)]
pub struct ADT {
    pub filename: String,
    pub path: PathBuf,
    pub x: u32,
    pub y: u32,

    pub mver: Option<types::shared::MVER>,
    pub mhdr: Option<types::adt::MHDR>,
    pub mcin: Option<types::adt::MCIN>,
    pub mtex: Option<types::adt::MTEX>,
    pub mmdx: Option<types::adt::MMDX>,
    pub mmid: Option<types::adt::MMID>,
    pub mwmo: Option<types::shared::MWMO>,
    pub mwid: Option<types::adt::MWID>,
    pub mddf: Option<types::adt::MDDF>,
    pub modf: Option<types::shared::MODF>,
    pub mcnk: Vec<types::adt::MCNK>,
}

fn parse_adt_file(path: PathBuf, mphd_flags: &types::wdt::MPHDFlags) -> Result<ADT, Error> {
    let filename = path.file_stem().ok_or(Error::File(path.clone()))?
        .to_string_lossy().to_string();

    let split: Vec<&str> = filename.split("_").collect();
    let x: u32 = split[split.len() - 2].parse().map_err(|_| Error::File(path.clone()))?;
    let y: u32 = split[split.len() - 1].parse().map_err(|_| Error::File(path.clone()))?;

    let mut file = File::open(&path)?;

    let mut parsed_adt = ADT {
        filename,
        path,
        x,
        y,
        ..Default::default()
    };

    loop {
        if let Some(chunk_wrapper) = file.read_le::<types::shared::ChunkWrapper>().ok() {
            match chunk_wrapper.token.as_str() {
                "MVER" => macros::parse_chunk!(types::shared::MVER, &chunk_wrapper.data, &mut parsed_adt.mver),
                "MHDR" => macros::parse_chunk!(types::adt::MHDR, &chunk_wrapper.data, &mut parsed_adt.mhdr),
                "MCIN" => macros::parse_chunk!(types::adt::MCIN, &chunk_wrapper.data, &mut parsed_adt.mcin),
                "MTEX" => macros::parse_chunk!(types::adt::MTEX, &chunk_wrapper.data, &mut parsed_adt.mtex),
                "MMDX" => macros::parse_chunk!(types::adt::MMDX, &chunk_wrapper.data, &mut parsed_adt.mmdx),
                "MMID" => macros::parse_chunk!(types::adt::MMID, &chunk_wrapper.data, &mut parsed_adt.mmid),
                "MWMO" => macros::parse_chunk!(types::shared::MWMO, &chunk_wrapper.data, &mut parsed_adt.mwmo),
                "MWID" => macros::parse_chunk!(types::adt::MWID, &chunk_wrapper.data, &mut parsed_adt.mwid),
                "MDDF" => macros::parse_chunk!(types::adt::MDDF, &chunk_wrapper.data, &mut parsed_adt.mddf),
                "MODF" => macros::parse_chunk!(types::shared::MODF, &chunk_wrapper.data, &mut parsed_adt.modf),
                "MCNK" => {
                    let chunk = parse_chunk_data_args::<types::adt::MCNK>(&chunk_wrapper.data, (mphd_flags.has_height_texturing, ))?;
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
    pub fn from_file(path: PathBuf, mphd_flags: &types::wdt::MPHDFlags) -> Result<Self, Error> {
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
