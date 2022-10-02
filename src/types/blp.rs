use core::fmt::Debug;
use std::{io::{Read, Seek, SeekFrom, BufWriter}, iter::zip, path::PathBuf, fs::File};

use binread::{BinRead, BinReaderExt, BinResult, ReadOptions};

use png::{BitDepth, ColorType, Transformations};

#[derive(Debug, BinRead)]
#[br(little)]
pub struct BLPPixel {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    _pad: u8,
}

#[derive(Clone, Copy, Debug, BinRead)]
#[br(little, repr = u8)]
pub enum AlphaCompression {
    DXT1 = 0,
    DXT3 = 1,
    ARGB8888 = 2,
    ARGB1555 = 3,
    ARGB4444 = 4,
    RGB565 = 5,
    A8 = 6,
    DXT5 = 7,
    UNSPECIFIED = 8,
    ARGB2565 = 9,
    BC5 = 11, // DXGI_FORMAT_BC5_UNORM 
    NumPixelFormats = 12, // (no idea if format=10 exists)
}

#[derive(Clone, Copy, Debug, BinRead)]
#[br(little, repr = u8)]
pub enum ColorEncoding {
    JPEG = 0, // not supported
    PALETTE = 1,
    DXT = 2,
    ARGB8888 = 3,
    ARGB8888_ = 4,    // same decompression, likely other PIXEL_FORMAT
}

#[derive(Debug)]
pub struct Mipmap {
    pub decompressed: Vec<u8>,
}

impl BinRead for Mipmap {
    type Args = (usize, ColorEncoding, u8, AlphaCompression, u32, u32, u32);

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        _: &ReadOptions,
        args: Self::Args,
    ) -> BinResult<Self> {
        let (
            layer,
            color_encoding,
            alpha_channel_bit_depth,
            alpha_compression,
            width,
            height,
            buffer_size,
        ) = args;

        match color_encoding {
            ColorEncoding::PALETTE => {
                Ok(Self { decompressed: Vec::new() })
            },
            ColorEncoding::DXT => {
                let format = match alpha_compression {
                    AlphaCompression::DXT1 => texpresso::Format::Bc1,
                    AlphaCompression::DXT3 => texpresso::Format::Bc2,
                    AlphaCompression::DXT5 => texpresso::Format::Bc3,
                    _ => panic!("Invalid alpha compression for DXT: {:?}", alpha_compression),
                };

                // Calculate the correct size of the compressed buffer.
                let valid_size = match alpha_compression {
                    AlphaCompression::DXT1 => ((width + 3) / 4) * ((height + 3) / 4) * 8,
                    AlphaCompression::DXT3 | AlphaCompression::DXT5 => ((width + 3) / 4) * ((height + 3) / 4) * 16,
                    _ => panic!("Invalid alpha compression for DXT: {:?}", alpha_compression),
                };

                let mut compressed = vec![0u8; valid_size as usize];
                let mut handle = reader.take(buffer_size.into());
                handle.read(&mut compressed)?;

                let width = width as usize;
                let height = height as usize;

                let mut decompressed = vec![0u8; 4 * width * height];
                format.decompress(&compressed, width, height, &mut decompressed);

                // let mut decompressed_reader = std::io::Cursor::new(decompressed);
                // let mut pixels: Vec<BLPPixel> = Vec::new();
                // for _ in 0..256 {
                //     pixels.push(decompressed_reader.read_le()?);
                // }

                // println!("{:?}", pixels);

                // let outfile = PathBuf::new()
                //     .with_file_name(format!("{}", layer))
                //     .with_extension("png");

                // let file = File::create(outfile).expect("Unable to create file");
                // let w = &mut BufWriter::new(file);

                // let mut encoder = png::Encoder::new(w, width as u32, height as u32);
                // encoder.set_color(ColorType::Rgba);
                // encoder.set_depth(BitDepth::Eight);
                // let mut writer = encoder.write_header().unwrap();

                // writer.write_image_data(&decompressed).unwrap();

                Ok(Self { decompressed })
            },
            ColorEncoding::ARGB8888 | ColorEncoding::ARGB8888_=> {
                Ok(Self { decompressed: Vec::new() })
            },
            _ => panic!("Unsupported format: {:?}", color_encoding),
        }
    }
}

#[derive(Debug)]
pub struct BLP {
    magic: u32,
    version: u32,
    pub color_encoding: ColorEncoding,
    pub alpha_channel_bit_depth: u8,
    pub alpha_compression: AlphaCompression,
    pub has_mips: u8,
    pub width: u32,
    pub height: u32,
    pub mipmaps: Vec<Mipmap>,
}

impl BinRead for BLP {
    type Args = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        _: &ReadOptions,
        args: Self::Args,
    ) -> BinResult<Self> {
        let magic: u32 = reader.read_le()?;
        let version: u32 = reader.read_le()?;

        let color_encoding: ColorEncoding = reader.read_le()?;
        let alpha_channel_bit_depth: u8 = reader.read_le()?;
        let alpha_compression: AlphaCompression = reader.read_le()?;

        let has_mips: u8 = reader.read_le()?;

        let width: u32 = reader.read_le()?;
        let height: u32 = reader.read_le()?;

        let mut mip_offsets: Vec<u32> = Vec::with_capacity(16);
        for _ in 0..16 {
            mip_offsets.push(reader.read_le()?);
        }

        let mut mip_sizes: Vec<u32> = Vec::with_capacity(16);
        for _ in 0..16 {
            mip_sizes.push(reader.read_le()?);
        }

        let mips = &mip_offsets.iter().filter(|o| **o != 0).count();

        let mut mipmaps: Vec<Mipmap> = Vec::with_capacity(*mips);
        for (i, (offset, size)) in zip(mip_offsets.into_iter(), mip_sizes.into_iter()).enumerate() {
            if offset != 0 && size != 0 {
                reader.seek(SeekFrom::Start(offset.into()))?;
                mipmaps.push(reader.read_le_args((i, color_encoding, alpha_channel_bit_depth, alpha_compression, width, height, size))?);
            }
        }

        Ok(Self {
            magic,
            version,
            
            color_encoding,
            alpha_channel_bit_depth,
            alpha_compression,

            has_mips,

            width,
            height,

            mipmaps,
        })
    }
}
