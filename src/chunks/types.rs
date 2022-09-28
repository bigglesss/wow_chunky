use core::fmt::Debug;
use std::io::{Read, Seek, SeekFrom};

use binread::{BinRead, BinReaderExt, BinResult, ReadOptions};

fn char_vec_to_string_le(v: &Vec<u8>, reversed: bool) -> String {
    if reversed {
        v.iter().rev().map(|v| char::from(*v)).collect::<String>()
    } else {
        v.iter().map(|v| char::from(*v)).collect::<String>()
    }
}

fn token_parse<R: Read + Seek>(reader: &mut R, _ro: &ReadOptions, _: ()) -> BinResult<String> {
    // Read 4 u8s into a buffer.
    let mut token = vec![0; 4];
    reader.read_exact(&mut token)?;

    // Parse by reversing the buffer and converting to a String.
    let string = char_vec_to_string_le(&token, true);

    Ok(string)
}

fn zero_terminated_strings<R: Read + Seek>(
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

fn read_until_end<R: Read + Seek, T: BinRead>(
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

#[derive(Debug, BinRead)]
#[br(little)]
pub struct CAaBox {
    min: C3Vector,
    max: C3Vector,
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct C3Vector {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct ChunkWrapper {
    #[br(parse_with = token_parse)]
    pub token: String,
    pub size: u32,
    #[br(count = size)]
    pub data: Vec<u8>,
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

#[derive(Debug, BinRead)]
#[br(little)]
pub struct MTEX {
    /*
    char filenames[0];              // zero-terminated strings with complete paths to textures. Referenced in MCLY.
    */
    #[br(parse_with = zero_terminated_strings)]
    pub filenames: Vec<String>,
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct MMDX {
    /*
    char filenames[0];              // zero-terminated strings with complete paths to models. Referenced in MMID.
    */
    #[br(parse_with = zero_terminated_strings)]
    pub filenames: Vec<String>,
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct MMID {
    /*
    uint32_t offsets[0];            // filename starting position in MMDX chunk. These entries are getting referenced in the MDDF chunk.
    */
    #[br(parse_with = read_until_end)]
    pub offsets: Vec<u32>,
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct MWMO {
    /*
    char filenames[0];              // zero-terminated strings with complete paths to models. Referenced in MWID.
    */
    #[br(parse_with = zero_terminated_strings)]
    pub filenames: Vec<String>,
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct MWID {
    /*
    uint32_t offsets[0];            // filename starting position in MWMO chunk. These entries are getting referenced in the MODF chunk.
    */
    #[br(parse_with = read_until_end)]
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
    C3Vectorⁱ position;           // This is relative to a corner of the map. Subtract 17066 from the non vertical values and you should start to see
                                     something that makes sense. You'll then likely have to negate one of the non vertical values in whatever
                                     coordinate system you're using to finally move it into place.
    C3Vectorⁱ rotation;           // degrees. This is not the same coordinate system orientation like the ADT itself! (see history.)
    uint16_t scale;               // 1024 is the default size equaling 1.0f.
    uint16_t flags;               // values from enum MDDFFlags.
    */
    pub name_id: u32,
    pub unique_id: u32,
    pub position: C3Vector,
    pub rotation: C3Vector,
    pub scale: u16,
    pub flags: MDDFFlags,
}
#[derive(Debug, BinRead)]
#[br(little)]
pub struct MDDF {
    #[br(parse_with = read_until_end)]
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
    C3Vectorⁱ position;
    C3Vectorⁱ rotation;           // same as in MDDF.
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
#[derive(Debug, BinRead)]
#[br(little)]
pub struct MODF {
    #[br(parse_with = read_until_end)]
    pub parts: Vec<MODFPart>,
}
#[derive(Debug)]
pub struct MCNK {
    pub flags: u32,

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

    pub position: C3Vector,
    pub ofs_mccv: u32,

    // Subchunks:
    pub mcvt: MCVT,
    pub mcnr: MCNR,
    pub mcly: MCLY,
    pub mcrf: MCRF,
    // pub mcal: MCAL,
}

// BinRead has to be manually implemented instead of derived for MCNK,
// as the MCAL subchunk requires flags from the MCLY subchunk,
// as well as flags from the WDT file.
impl BinRead for MCNK {
    type Args = ();

    fn args_default() -> Option<Self::Args> {
        Some(())
    }

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        options: &ReadOptions,
        args: Self::Args,
    ) -> BinResult<Self> {
        let flags: u32 = reader.read_le()?;

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

        let position: C3Vector = reader.read_le()?;
        let ofs_mccv: u32 = reader.read_le()?;
        let _unused: u32 = reader.read_le()?;
        let _unused2: u32 = reader.read_le()?;

        reader.seek(SeekFrom::Start(ofs_height.into()))?;
        let mcvt: MCVT = reader.read_le()?;

        reader.seek(SeekFrom::Start(ofs_normal.into()))?;
        let mcnr: MCNR = reader.read_le()?;

        reader.seek(SeekFrom::Start(ofs_layer.into()))?;
        let mcly: MCLY = reader.read_le_args((n_layers,))?;

        reader.seek(SeekFrom::Start(ofs_refs.into()))?;
        let mcrf: MCRF = reader.read_le_args((n_doodad_refs, n_map_obj_refs))?;

        reader.seek(SeekFrom::Start(ofs_alpha.into()))?;
        let mcal_layers: Vec<MCALLayer> = Vec::new();
        // let mcal = MCAL {
        //     layers: mcal_layers,
        // };

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
        })
    }
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct MCVT {
    #[br(count = 145)]
    height: Vec<f32>,
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct MCNREntry {
    x: i8,
    y: i8,
    z: i8,
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct MCNR {
    #[br(count = 145)]
    height: Vec<MCNREntry>,
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
        animate_45: bool,
        animate_90: bool,
        use_alpha: bool,
        alpha_compressed: bool,
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
    texture_id: u32,
    flags: mcly_flags::MCLYFlags,
    offset_in_mcal: u32,
    effect_id: u32,
}

#[derive(Debug, BinRead)]
#[br(little, import(n_layers: u32))]
pub struct MCLY {
    #[br(count = n_layers)]
    layers: Vec<MCLYLayer>,
}

#[derive(Debug, BinRead)]
#[br(little, import(n_doodad_refs: u32, n_map_obj_refs: u32))]
pub struct MCRF {
    #[br(count = n_doodad_refs)]
    doodad_refs: Vec<u32>,
    #[br(count = n_map_obj_refs)]
    n_map_obj_refs: Vec<u32>,
}

pub struct MCALLayer {
    texture_id: u32,
    flags: u32,
    offset_in_mcal: u32,
    effect_id: u32,
}
