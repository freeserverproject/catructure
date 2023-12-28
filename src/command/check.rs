use std::{
    path::PathBuf,
    collections::HashMap
};

use crate::{
    error::{Result, CatructureError},
    model::{structure::{self, Structure, PaletteBlock}, config::{Config, Blacklist}}, ascii_tree::Node
};

type Palette<'a> = HashMap::<&'a String, Vec<&'a structure::BlockPosition>>;

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
    let mut banned_palettes = Vec::<(String, Palette)>::new();

    // Static palette
    if let Some(palette) = &structure.palette {
        let banned_palette = structure.detect_blacklist_block(palette, &config.blacklist);
        if !banned_palette.is_empty() {
            banned_palettes.push((
                "Static palette".to_string(),
                banned_palette
            ));
        }
    }

    // Random palette
    if let Some(palettes) = &structure.palettes {
        for (index, palette) in palettes.iter().enumerate() {
            let banned_palette = structure.detect_blacklist_block(palette, &config.blacklist);
            if !banned_palette.is_empty() {
                banned_palettes.push((
                    format!("Random palette({})", index),
                    banned_palette
                ));
            }
        }
    }

    // banned_paletteが存在していたいたらASCII TREEに出力しエラーで終了
    // 存在してなければそのまま正常終了
    if !banned_palettes.is_empty() {
        let mut banned_palettes_tree = Node::new("Banned");

        for (palette_name, palette) in banned_palettes {
            let mut banned_palette_tree = Node::new(palette_name);

            for (block_name, block_positions) in palette.iter() {
                let mut banned_blocks_pos_node = Node::new((*block_name).clone());
                for block_pos in block_positions {
                    let block_pos = &block_pos.pos;
                    banned_blocks_pos_node.push(
                        format!("{} {} {}", block_pos[0], block_pos[1], block_pos[2])
                    );
                }

                banned_palette_tree.push(banned_blocks_pos_node);
            }

            banned_palettes_tree.push(banned_palette_tree);
        }


        Err(CatructureError::DetectBlacklistBlock(banned_palettes_tree.to_string()))
    } else {
        println!("File OK!");
        Ok(())
    }
}

trait BlockBlacklist {
    fn detect_blacklist_block<'a>(
        &'a self,
        palette: &'a Vec<PaletteBlock>,
        blacklist: &'a Blacklist
    ) -> Palette;
}

impl BlockBlacklist for Structure {
    fn detect_blacklist_block<'a>(
            &'a self,
            palette: &'a Vec<PaletteBlock>,
            blacklist: &'a Blacklist
    ) -> Palette {
        let mut bucket = Palette::new();

        for (index, block) in palette.iter().enumerate() {
            if blacklist.blocks.contains(&block.name) {
                let mut block_positions_tmp = Vec::new();

                self.blocks.iter().for_each(|block_pos| {
                    if block_pos.state as usize == index {
                        block_positions_tmp.push(block_pos);
                    }
                });

                bucket.entry(&block.name).or_default().extend(block_positions_tmp);
            }
        }

        bucket
    }
}
