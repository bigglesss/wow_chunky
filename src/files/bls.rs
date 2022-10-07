use core::fmt::Debug;
use std::{io::{SeekFrom, Cursor}, path::PathBuf};

use binread::{BinRead, BinReaderExt};

use crate::error::Error;



#[derive(BinRead, Debug)]
#[br(repr = u32)]
pub enum ParamType
{
    Vector4 = 0x0,            // C4Vector
    Matrix34 = 0x1,           // C34Matrix
    Matrix44 = 0x2,           // C44Matrix
    Texture = 0x3,            // used in terrain3*.bls, terrain4*.bls
    BumpMatrix = 0x4,         // used in terrain3*.bls, terrain4*.bls, Matrix 2x2 for bump
    Vec3 = 0x5,
    Vec2 = 0x6,
    Vec1 = 0x7,
    Matrix33 = 0x8,
    Struct = 0x9, /* no data */
    Array = 0xA, /* no data */
    Force32Bit = 0xFFFFFFFF,
}

#[derive(BinRead, Debug)]
pub struct BLSBlockParam {
    #[br(count = 64, map = |v: Vec<char>| v.into_iter().filter(|c| *c != '\0').collect())]
    pub name: String,
    binding: u32,
    #[br(count = 16)]
    float: Vec<f32>,
    param_type: ParamType,
    unk: u32,
    unk2: u32,
}

#[derive(BinRead, Debug)]
#[br(import(index: usize))]
pub struct BLSBlock {
    #[br(calc = index)]
    pub index: usize,

    pub constant_count: u32,
    #[br(count = constant_count)]
    pub constants: Vec<BLSBlockParam>,

    pub param_count: u32,
    #[br(count = param_count)]
    pub params: Vec<BLSBlockParam>,

    pub unk: u32,
    pub bytes: u32,

    #[br(count = bytes, map = |v: Vec<char>| v.into_iter().filter(|c| *c != '\0').collect())]
    pub code: String,
}

#[derive(Debug)]
pub struct BLS {
    pub token: String,
    version: u32,
    permutation_count: u32,
    blocks: Vec<BLSBlock>,
}

impl BinRead for BLS {
    type Args = ();

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        options: &binread::ReadOptions,
        args: Self::Args,
    ) -> binread::BinResult<Self> {
        let mut token: Vec<char> = Vec::new();
        for _ in 0..4 { token.push(reader.read_le()?) }
        let token: String = token.into_iter().collect();

        let version: u32 = reader.read_le()?;
        let permutation_count: u32 = reader.read_le()?;

        let offsets_n = match token.as_str() {
            "SPXG" => 12,
            "SVXG" => 6,
            _ => panic!("Found invalid shader type: {}", &token)
        };

        let mut offsets: Vec<u32> = Vec::new();
        for _ in 0..offsets_n { offsets.push(reader.read_le()?) };

        let mut blocks: Vec<BLSBlock> = Vec::new();
        for (i, offset) in offsets.into_iter().enumerate() {
            if offset != 0 {
                println!("Reading buffer {}", offset);
                reader.seek(SeekFrom::Start(offset.into()))?;
                blocks.push(reader.read_le_args((i,))?)
            }
        }

        Ok(Self {
            token,
            version,
            permutation_count,
            blocks,
        })
    }
}

impl TryFrom<PathBuf> for BLS {
    type Error = Error;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let file = std::fs::read(path).map_err(Error::IO)?;

        let mut cursor = Cursor::new(file);

        let parsed_blp: Self = cursor.read_le()?;
        return Ok(parsed_blp);
    }
}

