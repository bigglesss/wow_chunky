use core::fmt::Debug;

use binread::{BinRead, BinReaderExt};

const MPHD_FLAG_USES_GLOBAL_MAP_OBJ: u32 = 0x01;
const MPHD_FLAG_ADT_HAS_MCCV: u32 = 0x2;
const MPHD_FLAG_ADT_HAS_BIG_ALPHA: u32 = 0x4;
const MPHD_FLAG_ADT_HAS_DOODADS_SORTED_BY_SIZE: u32 = 0x8;
const MPHD_FLAG_LIGHTING_VERTICES: u32 = 0x10;
const MPHD_FLAG_UPSIDE_DOWN_GROUND: u32 = 0x20;
const MPHD_FLAG_UNK: u32 = 0x40;
const MPHD_FLAG_ADT_HAS_HEIGHT_TEXTURING: u32 = 0x80;


#[derive(Clone, Debug)]
pub struct MPHDFlags {
    pub has_height_texturing: bool,
}

impl BinRead for MPHDFlags {
    type Args = ();

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        options: &binread::ReadOptions,
        args: Self::Args,
    ) -> binread::BinResult<Self> {
        let i: u32 = reader.read_le()?;

        let unk = i & MPHD_FLAG_UNK == MPHD_FLAG_UNK;
        let has_height_texturing = i & MPHD_FLAG_ADT_HAS_HEIGHT_TEXTURING == MPHD_FLAG_ADT_HAS_HEIGHT_TEXTURING;

        Ok(Self {
            has_height_texturing: has_height_texturing || unk,
        })
    }
} 

#[derive(Clone, Debug, BinRead)]
#[br(little)]
pub struct MPHD {
    /*
        uint32_t version;
        uint32_t flags;
        uint32_t something;
        uint32_t unused[6];
    */
    pub version: u32,
    pub flags: MPHDFlags,
    pub _something: u32,
    pub _unused: u32,
}