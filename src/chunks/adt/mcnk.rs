use std::io::{Read, Seek, SeekFrom};

use bitvec::prelude::*;
use binread::{BinRead, ReadOptions, BinResult, BinReaderExt};

use crate::chunks::shared;

const MCNK_FLAG_HAS_MCSH: u32 = 0x01;
const MCNK_FLAG_IMPASS: u32 = 0x02;
const MCNK_FLAG_LQ_RIVER: u32 = 0x04;
const MCNK_FLAG_LQ_OCEAN: u32 = 0x08;
const MCNK_FLAG_LQ_MAGMA: u32 = 0x10;
const MCNK_FLAG_HAS_MCCV: u32 = 0x20;
const MCNK_FLAG_UNK: u32 = 0x40;
const MCNK_FLAG_DO_NOT_FIX_ALPHA_MAP: u32 = 0x200;

#[derive(Clone, Debug)]
pub struct MCNKFlags {
    pub has_mcsh: bool,
    pub impass: bool,
    pub lq_river: bool,
    pub lq_ocean: bool,
    pub lq_magma: bool,
    pub has_mccv: bool,

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

        let has_mcsh = i & MCNK_FLAG_HAS_MCCV == MCNK_FLAG_HAS_MCCV;
        let impass = i & MCNK_FLAG_IMPASS == MCNK_FLAG_IMPASS;
        let lq_river = i & MCNK_FLAG_LQ_RIVER == MCNK_FLAG_LQ_RIVER;
        let lq_ocean = i & MCNK_FLAG_LQ_OCEAN == MCNK_FLAG_LQ_OCEAN;
        let lq_magma = i & MCNK_FLAG_LQ_MAGMA == MCNK_FLAG_LQ_MAGMA;
        let has_mccv = i & MCNK_FLAG_HAS_MCCV == MCNK_FLAG_HAS_MCCV;

        let do_not_fix_alpha_map = i & MCNK_FLAG_DO_NOT_FIX_ALPHA_MAP == MCNK_FLAG_DO_NOT_FIX_ALPHA_MAP;

        Ok(Self {
            has_mcsh,
            impass,
            lq_river,
            lq_ocean,
            lq_magma,
            has_mccv,
            do_not_fix_alpha_map,
        })
    }
} 

#[derive(Clone, Debug)]
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
    pub holes_low_res: u16,

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
    pub mclq: MCLQ,
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
        let holes_low_res: u16 = reader.read_le()?;
        let _unk: u16 = reader.read_le()?;

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
                mcal_layers.push(mcal_reader.read_le_args::<MCALLayer>((args.0, l.flags.alpha_compressed, flags.do_not_fix_alpha_map))?);
            }
        }
        let mcal: MCAL = MCAL { layers: mcal_layers };

        reader.seek(SeekFrom::Start(ofs_liquid.into()))?;
        let mclq: MCLQ = reader.read_le_args((flags.lq_river, flags.lq_ocean, flags.lq_magma))?;

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
            mclq,
        })
    }
}

static ADT_SIZE: f32 = 533.0 + (1.0 / 3.0);
static QUAD_SIZE: f32 = ADT_SIZE / 128.0;

pub fn parse_heightmap(raw: Vec<f32>, offset:shared::C3Vector) -> Vec<shared::C3Vector> {
    // TODO: This currently uses the distant view heightmap, which causes some irregularities.
    // I also suspect the scaling is not the same as the blizz client, perhaps it need to be scaled up.
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

#[derive(Clone, Debug, BinRead)]
#[br(little, import(offset:shared::C3Vector))]
pub struct MCVT {
    #[br(count = 145, map = |raw: Vec<f32>| parse_heightmap(raw, offset))]
    pub heights: Vec<shared::C3Vector>,
}

#[derive(Clone, Debug, BinRead)]
#[br(little)]
pub struct MCNREntry {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

#[derive(Clone, Debug, BinRead)]
#[br(little)]
pub struct MCNR {
    #[br(count = 145)]
    pub normals: Vec<MCNREntry>,
}

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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug, BinRead)]
#[br(little)]
pub struct MCLYLayer {
    pub texture_id: u32,
    pub flags: MCLYFlags,
    pub offset_in_mcal: u32,
    pub effect_id: u32,
}

#[derive(Clone, Debug, BinRead)]
#[br(little, import(n_layers: u32))]
pub struct MCLY {
    #[br(count = n_layers)]
    pub layers: Vec<MCLYLayer>,
}

#[derive(Clone, Debug, BinRead)]
#[br(little, import(n_doodad_refs: u32, n_map_obj_refs: u32))]
pub struct MCRF {
    #[br(count = n_doodad_refs)]
    doodad_refs: Vec<u32>,
    #[br(count = n_map_obj_refs)]
    n_map_obj_refs: Vec<u32>,
}

#[derive(Clone, Debug)]
pub struct MCALLayer {
    pub alpha_map: Vec<u8>,
}

impl BinRead for MCALLayer {
    type Args = (bool, bool, bool);

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        options: &ReadOptions,
        args: Self::Args,
    ) -> BinResult<Self> {
        let (full_size, compressed, do_not_fix_alpha_map) = args;
        
        let decompressed = match compressed {
            true => {
                let mut data: Vec<u8> = Vec::new();
                // TODO: Ignore any bytes over 4096.
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
            while alpha_map.len() < 4096 {
                let byte: u8 = decompressed_reader.read_le()?;
                alpha_map.push(byte);
            }
        }
        else {
            // Chunks marked as half-size (MPHD flags *not* set) need to have each
            // u8 value splt in half, then converted back into u8s in order to construct
            // the full 64x64 alpha map.
            while alpha_map.len() < 4096 {
                let byte: u8 = decompressed_reader.read_le()?;
                let bit_slice = BitSlice::<u8, Lsb0>::from_element(&byte);
                let (left, right) = bit_slice.split_at(4);

                alpha_map.push(left.load::<u8>());
                alpha_map.push(right.load::<u8>());
            }
        }

        if !do_not_fix_alpha_map {
            for i in 0..alpha_map.len() {
                // Replace the last row with the previous rows value.
                if i > (4096 - 64) {
                    alpha_map[i] = alpha_map[i-64];
                }

                // Replace the last column with the previous columns value.
                if i > 0 && (i+1).rem_euclid(64) == 0 {
                    alpha_map[i] = alpha_map[i-1];
                }
            }
        }

        Ok(Self {
            alpha_map,
        })
    }
}

#[derive(Clone, Debug)]
pub struct MCAL {
    pub layers: Vec<MCALLayer>,
}

#[derive(BinRead, Clone, Debug)]
#[br(little)]
pub struct MCLQRiverVert {
    #[br(map = |d: char| d as u8)]
    pub depth: u8,
    #[br(map = |d: char| d as u8)]
    pub flow_0_pct: u8,
    #[br(map = |d: char| d as u8)]
    pub flow_1_pct: u8,
    #[br(map = |d: char| d as u8)]
    pub filler: u8,
    pub height: f32, 
}

#[derive(BinRead, Clone, Debug)]
#[br(little)]
pub struct MCLQOceanVert {
    #[br(map = |d: char| d as u8)]
    pub depth: u8,
    #[br(map = |d: char| d as u8)]
    pub foam: u8,
    #[br(map = |d: char| d as u8)]
    pub filler: u8,
    #[br(map = |d: char| d as u8)]
    pub wet: u8,
}

#[derive(BinRead, Clone, Debug)]
#[br(little, import(lq_river: bool, lq_ocean: bool, lq_magma: bool))]
pub struct MCLQ {
    pub height: shared::CRange,

    #[br(if(lq_river), count=9*9)]
    pub river_verts: Vec<MCLQRiverVert>,

    #[br(if(lq_ocean), count=9*9)]
    pub ocean_verts: Vec<MCLQOceanVert>,

    #[br(if(lq_magma), count=9*9)]
    pub magma_verts: Vec<MCLQRiverVert>,

    // Only parse if *any* liquid is present in the tile, and convert all chars into u8s.
    #[br(if(lq_river || lq_ocean || lq_magma), count=8*8, map = |d: Vec<char>| d.iter().map(|d| *d as u8).collect() )]
    pub tiles: Vec<u8>,
}
