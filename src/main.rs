use clap::Parser;

use std::path::PathBuf;

pub mod chunks;
pub mod error;

#[derive(Parser)]
struct Cli {
    #[clap(short, long, value_parser, value_name = "FILE")]
    file: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let file_path = cli.file;

    println!("{:#?}", file_path);
    if let Some(extension) = file_path.extension() {
        if extension == "adt" {
            let adt = chunks::parse_adt(file_path, chunks::types::MPHDFlags{ has_height_texturing: false }).unwrap();
            println!("{:#?}", adt.mcnk.last().unwrap().mcal);
        } else if extension == "wdt" {
            let wdt = chunks::parse_wdt(file_path).unwrap();
            println!("{:#?}", wdt.mphd);
        }
    }
}
