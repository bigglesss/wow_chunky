use clap::Parser;
use image::{RgbImage, Rgb, RgbaImage};

use std::path::PathBuf;

mod types;
mod error;
mod parser;

#[derive(Parser)]
struct Cli {
    #[clap(short, long, value_parser, value_name = "FILE")]
    file: PathBuf,

    #[clap(short, value_parser)]
    x: Option<u32>,
    #[clap(short, value_parser)]
    y: Option<u32>,

    #[clap(long, value_parser)]
    chunk_x: Option<u32>,
    #[clap(long, value_parser)]
    chunk_y: Option<u32>,

    #[clap(long, value_parser)]
    save_alphas: bool,
}

fn main() {
    let cli = Cli::parse();
    let file_path = cli.file;

    if let Some(extension) = file_path.extension() {
        if extension == "adt" {
            let adt = parser::ADT::from_file(file_path, &types::wdt::MPHDFlags{ has_height_texturing: false }).unwrap();
            println!("{:#?}", adt);
        } else if extension == "wdt" {
            if let (Some(x), Some(y)) = (cli.x, cli.y) {
                let adt = parser::ADT::from_wdt_file(file_path, x, y).unwrap();

                for c in adt.mcnk.iter() {
                    if Some(c.x) == cli.chunk_x && Some(c.y) == cli.chunk_y {
                        println!("{:?}", &c.holes_low_res);
                        
                        if cli.save_alphas {
                            for (i, data) in c.mcal.layers.iter().enumerate() {
                                let mut img = RgbImage::new(64, 64);

                                for (i, p) in data.alpha_map.iter().enumerate() {
                                    img.put_pixel((i % 64) as u32, (i / 64) as u32, Rgb([(*p * 17), 255, 255]));
                                }

                                let filename = format!("alpha_x{}_y{}_l{}.png", c.x, c.y, i);
                                img.save_with_format(filename, image::ImageFormat::Png).unwrap();
                            }
                        }
                    }
                }
            } else {
                let wdt = parser::WDT::from_file(file_path).unwrap();
                println!("{:#?}", wdt);
            }
        } else if extension == "blp" {
            let blp = parser::parse_blp(file_path.clone()).unwrap();

            for (i, data) in blp.mipmaps.iter().enumerate() {
                let img = RgbaImage::from_raw(blp.width, blp.height, data.decompressed.clone()).unwrap();
                let name = file_path.file_stem().unwrap();
                img.save_with_format(format!("{:?}_{}.png", name, i), image::ImageFormat::Png).unwrap();
            }
        } else if extension == "bls" {
            let bls = parser::parse_bls(file_path).unwrap();
            println!("{:#?}", bls);
        }
    }
}
