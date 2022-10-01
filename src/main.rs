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

    println!("{:#?}", file_path);
    if let Some(extension) = file_path.extension() {
        if extension == "adt" {
            let adt = parser::parse_adt(file_path, types::chunks::MPHDFlags{ has_height_texturing: false }).unwrap();
            println!("{:#?}", adt.mcnk.last().unwrap().mcvt);
        } else if extension == "wdt" {
            let wdt = parser::parse_wdt(file_path).unwrap();
            println!("{:#?}", wdt.mphd);
        }
    }
}
