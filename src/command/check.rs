use ascii_tree::{Tree, write_tree};
use flate2::read::GzDecoder;
use std::{
    io::Read, path::PathBuf,
    fs::File
};

use crate::{
    error::{Result, CatructureError},
    model::{structure, config::Config}
};

#[derive(Debug, clap::Args)]
pub struct Arg {
    file: PathBuf,

    #[arg(
        short = 'c',
        long = "config",
        default_value = "catructure.toml"
    )]
    config: PathBuf
}

pub fn run(arg: Arg) -> Result<()> {
    let config = Config::read(arg.config)?;

    // ファイル読み込み & デシリアライズ
    let file = File::open(&arg.file).unwrap();
    let mut decoder = GzDecoder::new(file);
    let mut structure = vec![];
    decoder.read_to_end(&mut structure).unwrap();

    let structure = fastnbt::from_bytes::<structure::Structure>(&structure)
        .expect("Failed serialize structure.");

    // Paletteにblacklistのブロックが存在するか確認し存在したら随時追加
    let mut blocked_blocks = Vec::<(&structure::PaletteBlock, Vec<&structure::BlockPosition>)>::new();

    if let Some(palette) = &structure.palette {
        for (index, block) in palette.iter().enumerate() {
            if config.blacklist.blocks.contains(&block.name) {
                let mut block_positions = Vec::new();

                structure.blocks.iter().for_each(|block_pos| {
                    if block_pos.state as usize == index {
                        block_positions.push(block_pos);
                    }
                });

                blocked_blocks.push((
                    block,
                    block_positions
                ));
            }
        }
    }

    // 
    if !blocked_blocks.is_empty() {
        let mut blocked: Vec<Tree> = Vec::new();
        for (palette_block, block_positions) in blocked_blocks {
            let mut positions: Vec<Tree> = Vec::new();
            for block_pos in block_positions {
                let block_pos = &block_pos.pos;
                positions.push(Tree::Leaf(vec![
                    format!("{} {} {}", block_pos[0], block_pos[1], block_pos[2])
                ]));
            }

            let blocked_blocks_node = Tree::Node(format!("{}", palette_block.name), positions);
            blocked.push(blocked_blocks_node);
        }

        let mut blocked_blocks_tree_string = String::new();
        write_tree(&mut blocked_blocks_tree_string, &Tree::Node(String::from("Blocked"), blocked)).expect("Failed write tree.");

        Err(CatructureError::DetectBlacklistBlock(blocked_blocks_tree_string))
    } else {
        println!("File OK!");
        Ok(())
    }

    // println!(
    //     "{:?}",
    //     structure
    // );
}
