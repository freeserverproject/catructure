use fastnbt::error::Result as FastNBTResult;
use flate2::read::GzDecoder;
use std::{
    io::Read, path::PathBuf, fs,
};


use crate::model::structure;

#[derive(Debug, clap::Args)]
pub struct Arg {
    file: PathBuf,

    #[arg(
        short = 'c',
        long = "config",
        default_value = "blacklist.toml"
    )]
    config: PathBuf
}

pub fn run(arg: Arg) {
    let file = fs::File::open(&arg.file).unwrap();
    let mut decoder = GzDecoder::new(file);
    let mut structure = vec![];
    decoder.read_to_end(&mut structure).unwrap();

    let structure: FastNBTResult<structure::Structure> = fastnbt::from_bytes(&structure);

    println!(
        "{:?}",
        structure
    );
}
