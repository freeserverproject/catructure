use ascii_tree::{Tree, write_tree};
use clap::builder::Str;
use std::{path::PathBuf, collections::HashMap};

use crate::{
    error::{Result, CatructureError},
    model::{structure::{self, Structure}, config::Config}
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
    let config = Config::load(arg.config)?;
    let structure = Structure::load(arg.file)?;

    // Paletteにblacklistのブロックが存在するか確認し存在したら随時追加
    let mut banned_blocks = HashMap::<&String, Vec<&structure::BlockPosition>>::new();

    if let Some(palette) = &structure.palette {
        for (index, block) in palette.iter().enumerate() {
            if config.blacklist.blocks.contains(&block.name) {
                let mut block_positions_tmp = Vec::new();

                structure.blocks.iter().for_each(|block_pos| {
                    if block_pos.state as usize == index {
                        block_positions_tmp.push(block_pos);
                    }
                });

                if let Some(block_positions) = banned_blocks.get_mut(&block.name) {
                    block_positions.extend(block_positions_tmp);
                } else {
                    banned_blocks.insert(
                        &block.name,
                        block_positions_tmp
                    );
                }
            }
        }
    }

    if !banned_blocks.is_empty() {
        let mut blocked: Vec<Tree> = Vec::new();
        for (block_name, block_positions) in banned_blocks {
            let mut positions: Vec<Tree> = Vec::new();
            for block_pos in block_positions {
                let block_pos = &block_pos.pos;
                positions.push(Tree::Leaf(vec![
                    format!("{} {} {}", block_pos[0], block_pos[1], block_pos[2])
                ]));
            }

            let banned_blocks_node = Tree::Node(format!("{}", block_name), positions);
            blocked.push(banned_blocks_node);
        }

        let mut banned_blocks_tree_string = String::new();
        write_tree(&mut banned_blocks_tree_string, &Tree::Node(String::from("Blocked"), blocked)).expect("Failed write tree.");

        Err(CatructureError::DetectBlacklistBlock(banned_blocks_tree_string))
    } else {
        println!("File OK!");
        Ok(())
    }
}
