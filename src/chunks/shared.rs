use core::fmt::Debug;
use std::io::{Read, Seek};
use binread::{BinRead, BinReaderExt, BinResult, ReadOptions};

#[derive(Clone, Debug, BinRead)]
#[br(little)]
pub struct CRange {
    pub min: f32,
    pub max: f32,
}

#[derive(Clone, Debug, BinRead)]
#[br(little)]
pub struct CAaBox {
    pub min: C3Vector,
    pub max: C3Vector,
}

#[derive(Clone, Copy, Debug, BinRead)]
#[br(little)]
pub struct C3Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub fn char_vec_to_string_le(v: &Vec<u8>, reversed: bool) -> String {
    if reversed {
        v.iter().rev().map(|v| char::from(*v)).collect::<String>()
    } else {
        v.iter().map(|v| char::from(*v)).collect::<String>()
    }
}


pub fn zero_terminated_strings<R: Read + Seek>(
    reader: &mut R,
    _: &ReadOptions,
    _: (),
) -> BinResult<Vec<String>> {
    let mut strings: Vec<String> = Vec::new();
    let mut string_buf: Vec<u8> = Vec::new();

    loop {
        match reader.read_le::<u8>() {
            Ok(v) => {
                if v != u8::MIN {
                    string_buf.push(v);
                } else {
                    strings.push(char_vec_to_string_le(&string_buf, false));
                    string_buf.clear();
                }
            }
            Err(_) => break,
        }
    }

    Ok(strings)
}

pub fn read_until_end<R: Read + Seek, T: BinRead>(
    reader: &mut R,
    _: &ReadOptions,
    _: (),
) -> BinResult<Vec<T>> {
    let mut values: Vec<T> = Vec::new();

    loop {
        match reader.read_le::<T>() {
            Ok(v) => {
                values.push(v);
            }
            Err(_) => break,
        }
    }

    Ok(values)
}

pub fn token_parse<R: Read + Seek>(reader: &mut R, _ro: &ReadOptions, _: ()) -> BinResult<String> {
    // Read 4 u8s into a buffer.
    let mut token = vec![0; 4];
    reader.read_exact(&mut token)?;

    // Parse by reversing the buffer and converting to a String.
    let string = char_vec_to_string_le(&token, true);

    Ok(string)
}

#[derive(Clone, Debug, BinRead)]
#[br(little)]
pub struct ChunkWrapper {
    #[br(parse_with = token_parse)]
    pub token: String,
    pub size: u32,
    #[br(count = size)]
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, BinRead)]
#[br(little)]
pub struct MVER {
    /*
        uint32_t version;
    */
    pub version: u32,
}

#[derive(Clone, Debug, BinRead)]
#[br(little)]
pub struct MWMO {
    /*
    char filenames[0];              // zero-terminated strings with complete paths to models. Referenced in MWID.
    */
    #[br(parse_with = zero_terminated_strings)]
    pub filenames: Vec<String>,
}

#[derive(Clone, Debug, BinRead)]
#[br(little, repr = u16)]
pub enum MODFFlags {
    NONE = 0,
    DESTROYABLE = 1,
}

#[derive(Clone, Debug, BinRead)]
#[br(little)]
pub struct MODFPart {
    /*
    uint32_t nameId;              // references an entry in the MWID chunk, specifying the model to use.
    uint32_t uniqueId;            // this ID should be unique for all ADTs currently loaded. Best, they are unique for the whole map.
   shared::C3Vectorⁱ position;
   shared::C3Vectorⁱ rotation;           // same as in MDDF.
    CAaBoxⁱ extents;              // position plus the transformed wmo bounding box. used for defining if they are rendered as well as collision.
    uint16_t flags;               // values from enum MODFFlags.
    uint16_t doodadSet;           // which WMO doodad set is used. Traditionally references WMO#MODS_chunk, if modf_use_sets_from_mwds is set, references #MWDR_.28Shadowlands.2B.29
    uint16_t nameSet;             // which WMO name set is used. Used for renaming goldshire inn to northshire inn while using the same model.
    uint16_t scale;               // Legion+: scale, 1024 means 1 (same as MDDF). Padding in 0.5.3 alpha.
    */
    pub name_id: u32,
    pub unique_id: u32,
    pub position: C3Vector,
    pub rotation: C3Vector,
    pub extends: CAaBox,
    pub scale: u16,
    pub flags: MODFFlags,
    pub doodat_set: u16,
    pub name_set: u16,
}

#[derive(Clone, Debug, BinRead)]
#[br(little)]
pub struct MODF {
    #[br(parse_with = read_until_end)]
    pub parts: Vec<MODFPart>,
}

