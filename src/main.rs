use clap::Parser;

use std::path::PathBuf;

mod chunks;
mod error;

#[derive(Parser)]
struct Cli {
    #[clap(short, long, value_parser, value_name = "FILE")]
    file: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let file_path = cli.file;

    let adt = chunks::parse_adt(file_path).unwrap();
    println!("{:#?}", adt.mcnk.last().unwrap());
}
