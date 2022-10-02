use core::fmt::Debug;
use std::io::{Read, Seek, SeekFrom};

use bitvec::prelude::*;
use binread::{BinRead, BinReaderExt, BinResult, ReadOptions};

pub mod shared;

const MPHD_FLAG_USES_GLOBAL_MAP_OBJ: u32 = 0x01;
const MPHD_FLAG_ADT_HAS_MCCV: u32 = 0x2;
const MPHD_FLAG_ADT_HAS_BIG_ALPHA: u32 = 0x4;
const MPHD_FLAG_ADT_HAS_DOODADS_SORTED_BY_SIZE: u32 = 0x8;
const MPHD_FLAG_LIGHTING_VERTICES: u32 = 0x10;
const MPHD_FLAG_UPSIDE_DOWN_GROUND: u32 = 0x20;
const MPHD_FLAG_UNK: u32 = 0x40;
const MPHD_FLAG_ADT_HAS_HEIGHT_TEXTURING: u32 = 0x80;

#[derive(Debug)]
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

#[derive(Debug, BinRead)]
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

#[derive(Debug, BinRead)]
#[br(little)]
pub struct MAINTile {
    pub has_adt: u32,
    pub flag_loaded: u32,
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct MAIN {
    #[br(count = 4096)]
    pub tiles: Vec<MAINTile>,
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct MVER {
    /*
        uint32_t version;
    */
    pub version: u32,
}

#[derive(Debug, BinRead)]
#[br(little, repr = u32)]
pub enum MHDRFlags {
    NONE = 0,
    MFBO = 1,
    NORTHREND = 2,
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct MHDR {
    /*
    enum MHDRFlags {
        mhdr_MFBO = 1,                // contains a MFBO chunk.
        mhdr_northrend = 2,           // is set for some northrend ones.
    };
    uint32_t flags;
    uint32_t mcin;                 // MCIN*, Cata+: obviously gone. probably all offsets gone, except mh2o(which remains in root file).
    uint32_t mtex;                 // MTEX*
    uint32_t mmdx;                 // MMDX*
    uint32_t mmid;                 // MMID*
    uint32_t mwmo;                 // MWMO*
    uint32_t mwid;                 // MWID*
    uint32_t mddf;                 // MDDF*
    uint32_t modf;                 // MODF*
    uint32_t mfbo;                 // MFBO*   this is only set if flags & mhdr_MFBO.
    uint32_t mh2o;                 // MH2O*
    uint32_t mtxf;                 // MTXF*
    uint8_t mamp_value;             // Cata+, explicit MAMP chunk overrides data
    uint8_t padding[3];
    uint32_t unused[3];
    */
    pub flags: MHDRFlags,
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct MCIN {
    /*
    uint32_t offset;               // absolute offset.
    uint32_t size;                 // the size of the MCNK chunk, this is refering to.
    uint32_t flags;                // always 0. only set in the client., FLAG_LOADED = 1
    union
    {
        char pad[4];
        uint32_t asyncId;            // not in the adt file. client use only
    };
    */
}

#[derive(Clone, Debug, BinRead)]
#[br(little)]
pub struct MTEX {
    /*
    char filenames[0];              // zero-terminated strings with complete paths to textures. Referenced in MCLY.
    */
    #[br(parse_with = shared::zero_terminated_strings)]
    pub filenames: Vec<String>,
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct MMDX {
    /*
    char filenames[0];              // zero-terminated strings with complete paths to models. Referenced in MMID.
    */
    #[br(parse_with = shared::zero_terminated_strings)]
    pub filenames: Vec<String>,
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct MMID {
    /*
    uint32_t offsets[0];            // filename starting position in MMDX chunk. These entries are getting referenced in the MDDF chunk.
    */
    #[br(parse_with = shared::read_until_end)]
    pub offsets: Vec<u32>,
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct MWMO {
    /*
    char filenames[0];              // zero-terminated strings with complete paths to models. Referenced in MWID.
    */
    #[br(parse_with = shared::zero_terminated_strings)]
    pub filenames: Vec<String>,
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct MWID {
    /*
    uint32_t offsets[0];            // filename starting position in MWMO chunk. These entries are getting referenced in the MODF chunk.
    */
    #[br(parse_with = shared::read_until_end)]
    pub offsets: Vec<u32>,
}

#[derive(Debug, BinRead)]
#[br(little, repr = u16)]
pub enum MDDFFlags {
    NONE = 0,
    BIODOME = 1,
    SHRUBBERY = 2,
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct MDDFPart {
    /*
    uint32_t nameId;              // references an entry in the MMID chunk, specifying the model to use.
                                     if flag mddf_entry_is_filedata_id is set, a file data id instead, ignoring MMID.
    uint32_t uniqueId;            // this ID should be unique for all ADTs currently loaded. Best, they are unique for the whole map. Blizzard has
                                     these unique for the whole game.
   shared::C3Vectorⁱ position;           // This is relative to a corner of the map. Subtract 17066 from the non vertical values and you should start to see
                                     something that makes sense. You'll then likely have to negate one of the non vertical values in whatever
                                     coordinate system you're using to finally move it into place.
   shared::C3Vectorⁱ rotation;           // degrees. This is not the same coordinate system orientation like the ADT itself! (see history.)
    uint16_t scale;               // 1024 is the default size equaling 1.0f.
    uint16_t flags;               // values from enum MDDFFlags.
    */
    pub name_id: u32,
    pub unique_id: u32,
    pub position: shared::C3Vector,
    pub rotation: shared::C3Vector,
    pub scale: u16,
    pub flags: MDDFFlags,
}
#[derive(Debug, BinRead)]
#[br(little)]
pub struct MDDF {
    #[br(parse_with = shared::read_until_end)]
    pub parts: Vec<MDDFPart>,
}

#[derive(Debug, BinRead)]
#[br(little, repr = u16)]
pub enum MODFFlags {
    NONE = 0,
    DESTROYABLE = 1,
}

#[derive(Debug, BinRead)]
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
    pub position: shared::C3Vector,
    pub rotation: shared::C3Vector,
    pub extends: shared::CAaBox,
    pub scale: u16,
    pub flags: MODFFlags,
    pub doodat_set: u16,
    pub name_set: u16,
}
#[derive(Debug, BinRead)]
#[br(little)]
pub struct MODF {
    #[br(parse_with = shared::read_until_end)]
    pub parts: Vec<MODFPart>,
}

const MCNK_FLAG_DO_NOT_FIX_ALPHA_MAP: u32 = 0x200;

#[derive(Debug)]
pub struct MCNKFlags {
    pub do_not_fix_alpha_map: bool,
}

impl BinRead for MCNKFlags {
    type Args = ();

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        options: &binread::ReadOptions,
        args: Self::Args,
    ) -> binread::BinResult<Self> {
        let i: u32 = reader.read_le()?;

        let do_not_fix_alpha_map = i & MCNK_FLAG_DO_NOT_FIX_ALPHA_MAP == MCNK_FLAG_DO_NOT_FIX_ALPHA_MAP;

        Ok(Self {
            do_not_fix_alpha_map
        })
    }
} 

#[derive(Debug)]
pub struct MCNK {
    pub flags: MCNKFlags,

    pub x: u32,
    pub y: u32,

    pub n_layers: u32,
    pub n_doodad_refs: u32,

    // Offsets.
    ofs_height: u32,
    ofs_normal: u32,
    ofs_layer: u32,
    ofs_refs: u32,
    ofs_alpha: u32,
    size_alpha: u32,
    ofs_shadow: u32,
    size_shadow: u32,

    pub area_id: u32,
    pub n_map_obj_refs: u32,
    pub holes_low_res: u32,

    pub low_res_texture_map: Vec<u16>,

    pub doodad_stencil: Vec<u8>,

    pub ofs_snd_emitters: u32,
    pub n_snd_emitters: u32,

    pub ofs_liquid: u32,
    pub size_liquid: u32,

    pub position: shared::C3Vector,
    pub ofs_mccv: u32,

    // Subchunks:
    pub mcvt: MCVT,
    pub mcnr: MCNR,
    pub mcly: MCLY,
    pub mcrf: MCRF,
    pub mcal: MCAL,
}

// BinRead has to be manually implemented instead of derived for MCNK,
// as the MCAL subchunk requires flags from the MCLY subchunk,
// as well as flags from the WDT file.
impl BinRead for MCNK {
    type Args = (bool, );

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        _: &ReadOptions,
        args: Self::Args,
    ) -> BinResult<Self> {
        let flags: MCNKFlags = reader.read_le()?;

        let x: u32 = reader.read_le()?;
        let y: u32 = reader.read_le()?;

        let n_layers: u32 = reader.read_le()?;
        let n_doodad_refs: u32 = reader.read_le()?;

        let ofs_height: u32 = reader.read_le()?;
        let ofs_normal: u32 = reader.read_le()?;
        let ofs_layer: u32 = reader.read_le()?;
        let ofs_refs: u32 = reader.read_le()?;
        let ofs_alpha: u32 = reader.read_le()?;
        let size_alpha: u32 = reader.read_le()?;
        let ofs_shadow: u32 = reader.read_le()?;
        let size_shadow: u32 = reader.read_le()?;

        let area_id: u32 = reader.read_le()?;
        let n_map_obj_refs: u32 = reader.read_le()?;
        let holes_low_res: u32 = reader.read_le()?;

        // We have to manually create a ReadOptions and mutate it, as it is non-exhaustative.
        let mut vec_options = ReadOptions::default();
        vec_options.count = Some(8);

        let low_res_texture_map: Vec<u16> = BinRead::read_options(reader, &vec_options, ())?;
        let doodad_stencil: Vec<u8> = BinRead::read_options(reader, &vec_options, ())?;

        let ofs_snd_emitters: u32 = reader.read_le()?;
        let n_snd_emitters: u32 = reader.read_le()?;

        let ofs_liquid: u32 = reader.read_le()?;
        let size_liquid: u32 = reader.read_le()?;

        let position: shared::C3Vector = reader.read_le()?;
        let ofs_mccv: u32 = reader.read_le()?;
        let _unused: u32 = reader.read_le()?;
        let _unused2: u32 = reader.read_le()?;

        reader.seek(SeekFrom::Start(ofs_height.into()))?;
        let mcvt: MCVT = reader.read_le_args((position, ))?;

        reader.seek(SeekFrom::Start(ofs_normal.into()))?;
        let mcnr: MCNR = reader.read_le()?;

        reader.seek(SeekFrom::Start(ofs_layer.into()))?;
        let mcly: MCLY = reader.read_le_args((n_layers,))?;

        reader.seek(SeekFrom::Start(ofs_refs.into()))?;
        let mcrf: MCRF = reader.read_le_args((n_doodad_refs, n_map_obj_refs))?;

        reader.seek(SeekFrom::Start(ofs_alpha.into()))?;
        let mut mcal_subchunk = vec![0; size_alpha.try_into().unwrap()];
        reader.read_exact(&mut mcal_subchunk)?;

        let mut mcal_reader = std::io::Cursor::new(mcal_subchunk);
        let mut mcal_layers: Vec<MCALLayer> = Vec::new();
        for l in mcly.layers.iter() {
            if l.flags.use_alpha {
                mcal_layers.push(mcal_reader.read_le_args::<MCALLayer>((args.0, l.flags.alpha_compressed,))?);
            }
        }
        let mcal: MCAL = MCAL { layers: mcal_layers };

        Ok(Self {
            flags,

            x,
            y,

            n_layers,
            n_doodad_refs,

            ofs_height,
            ofs_normal,
            ofs_layer,
            ofs_refs,
            ofs_alpha,
            size_alpha,
            ofs_shadow,
            size_shadow,

            area_id,
            n_map_obj_refs,
            holes_low_res,

            low_res_texture_map,
            doodad_stencil,

            ofs_snd_emitters,
            n_snd_emitters,

            ofs_liquid,
            size_liquid,

            position,
            ofs_mccv,

            mcvt,
            mcnr,
            mcly,
            mcrf,
            mcal,
        })
    }
}

static ADT_SIZE: f32 = 533.0 + (1.0 / 3.0);
static QUAD_SIZE: f32 = ADT_SIZE / 128.0;

pub fn parse_heightmap(raw: Vec<f32>, offset:shared::C3Vector) -> Vec<shared::C3Vector> {
    let mut parsed: Vec<shared::C3Vector> = Vec::new();
    for (i, height) in raw.iter().enumerate() {
        // if i % 17 > 8, this is the inner part of a quad
        let inner = (i % 17) > 8;

        let i: i32 = i.try_into().unwrap();

        let x: i32 = if inner {(i - 9) % 17} else {i % 17};
        let y: i32 = i / 17;

        let inner_offset: f32 = if inner {QUAD_SIZE / 2.0} else {0.0};

        let z: f32 = offset.z + height;

        let world_x = offset.x - ((y as f32) * QUAD_SIZE) - inner_offset;
        let world_y = offset.y - ((x as f32) * QUAD_SIZE) - inner_offset;

        parsed.push(shared::C3Vector { x: world_x, y: world_y, z });
    }

    parsed
}

#[derive(Debug, BinRead)]
#[br(little, import(offset:shared::C3Vector))]
pub struct MCVT {
    #[br(count = 145, map = |raw: Vec<f32>| parse_heightmap(raw, offset))]
    pub heights: Vec<shared::C3Vector>,
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct MCNREntry {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct MCNR {
    #[br(count = 145)]
    pub normals: Vec<MCNREntry>,
}

// TODO: Add other flag modules.
mod mcly_flags {
    use binread::BinRead;
    use binread::BinReaderExt;

    const MCLY_FLAG_ANIMATE_45: u32 = 0x01;
    const MCLY_FLAG_ANIMATE_90: u32 = 0x2;
    const MCLY_FLAG_ANIMATE_180: u32 = 0x4;
    const MCLY_FLAG_ANIM_FAST: u32 = 0x8;
    const MCLY_FLAG_ANIM_FASTER: u32 = 0x10;
    const MCLY_FLAG_ANIM_FASTEST: u32 = 0x20;
    const MCLY_FLAG_ANIMATE: u32 = 0x40;
    const MCLY_FLAG_GLOW: u32 = 0x80;
    const MCLY_FLAG_USE_ALPHA: u32 = 0x100;
    const MCLY_FLAG_ALPHA_COMPRESSED: u32 = 0x200;
    const MCLY_FLAG_REFLECTION: u32 = 0x400;

    #[derive(Debug)]
    pub struct MCLYFlags {
        pub animate_45: bool,
        pub animate_90: bool,
        pub use_alpha: bool,
        pub alpha_compressed: bool,
    }

    impl BinRead for MCLYFlags {
        type Args = ();

        fn read_options<R: std::io::Read + std::io::Seek>(
            reader: &mut R,
            options: &binread::ReadOptions,
            args: Self::Args,
        ) -> binread::BinResult<Self> {
            let i: u32 = reader.read_le()?;

            let use_alpha = i & MCLY_FLAG_USE_ALPHA == MCLY_FLAG_USE_ALPHA;
            let alpha_compressed = i & MCLY_FLAG_ALPHA_COMPRESSED == MCLY_FLAG_ALPHA_COMPRESSED;

            Ok(Self {
                animate_45: false,
                animate_90: false,
                use_alpha,
                alpha_compressed,
            })
        }
    } 
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct MCLYLayer {
    pub texture_id: u32,
    pub flags: mcly_flags::MCLYFlags,
    pub offset_in_mcal: u32,
    pub effect_id: u32,
}

#[derive(Debug, BinRead)]
#[br(little, import(n_layers: u32))]
pub struct MCLY {
    #[br(count = n_layers)]
    pub layers: Vec<MCLYLayer>,
}

#[derive(Debug, BinRead)]
#[br(little, import(n_doodad_refs: u32, n_map_obj_refs: u32))]
pub struct MCRF {
    #[br(count = n_doodad_refs)]
    doodad_refs: Vec<u32>,
    #[br(count = n_map_obj_refs)]
    n_map_obj_refs: Vec<u32>,
}

#[derive(Debug)]
pub struct MCALLayer {
    pub alpha_map: Vec<u8>,
}

impl BinRead for MCALLayer {
    type Args = (bool, bool, );

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        options: &ReadOptions,
        args: Self::Args,
    ) -> BinResult<Self> {
        let (full_size, compressed) = args;
        
        let decompressed = match compressed {
            true => {
                let mut data: Vec<u8> = Vec::new();
                loop {
                    if let Ok(count_and_mode) = reader.read_be::<u8>() {
                        let fill = count_and_mode & 0x80 == 0x80;
                        let count = count_and_mode & 0x7F;

                        // fill
                        if fill {
                            let value = reader.read_le::<u8>()?;
                            for _ in 0..count {
                                data.push(value);
                            }
                        }
                        // copy
                        else {
                            for _ in 0..count {
                                let value = reader.read_le::<u8>()?;
                                data.push(value);
                            }
                        }
                    } else {
                        break;
                    }
                }

                data 
            },
            _ => {
                let mut data = if full_size {vec![0; 4096]} else {vec![0; 2048]};
                reader.read_exact(&mut data)?;

                data
            },
        };

        let mut decompressed_reader = std::io::Cursor::new(decompressed);
        let mut alpha_map: Vec<u8> = Vec::new();
        if full_size {
            while alpha_map.len() < 2048 {
                let byte: u8 = decompressed_reader.read_le()?;
                alpha_map.push(byte);
            }
        } else {
            while alpha_map.len() < 4096 {
                let byte: u8 = decompressed_reader.read_le()?;
                let bit_slice = BitSlice::<u8, Lsb0>::from_element(&byte);
                let (left, right) = bit_slice.split_at(4);

                alpha_map.push(left.load::<u8>());
                alpha_map.push(right.load::<u8>());
            }
        }

        Ok(Self {
            alpha_map,
        })
    }
}

#[derive(Debug)]
pub struct MCAL {
    pub layers: Vec<MCALLayer>,
}
