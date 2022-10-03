use clap::Parser;

use std::path::PathBuf;

mod types;
mod error;
mod parser;

#[derive(Parser)]
struct Cli {
    #[clap(short, long, value_parser, value_name = "FILE")]
    file: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let file_path = cli.file;

    if let Some(extension) = file_path.extension() {
        if extension == "adt" {
            let adt = parser::adt::ADT::from_file(&file_path, types::chunks::MPHDFlags{ has_height_texturing: false }).unwrap();
            println!("{:#?}", adt);
        } else if extension == "wdt" {
            let wdt = parser::wdt::WDT::from_file(&file_path).unwrap();
            println!("{:#?}", wdt);
        } else if extension == "blp" {
            let blp = parser::parse_blp(file_path).unwrap();
            println!("{:#?}", blp);
        } else if extension == "bls" {
            let bls = parser::parse_bls(file_path).unwrap();
            println!("{:#?}", bls);
        }
    }
}
