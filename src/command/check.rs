use std::{
    path::PathBuf,
    collections::HashMap
};

use crate::{
    error::{Result, CatructureError},
    model::{structure::{Structure, PaletteBlock}, config::{Config, Blacklist}}, ascii_tree::Node
};

type Target<'a> = HashMap::<&'a String, Vec<String>>;

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
    let mut banned_targets = Vec::<(String, Target)>::new();

    // Static palette
    if let Some(palette) = &structure.palette {
        let banned_palette = structure.detect_blacklist_block(palette, &config.blacklist);
        if !banned_palette.is_empty() {
            banned_targets.push((
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
                banned_targets.push((
                    format!("Random palette({})", index),
                    banned_palette
                ));
            }
        }
    }

    // banned_paletteが存在していたいたらASCII TREEに出力しエラーで終了
    // 存在してなければそのまま正常終了
    if !banned_targets.is_empty() {
        let mut banned_targets_tree = Node::new("Banned");

        for (target_name, targe) in banned_targets {
            let mut banned_target_tree = Node::new(target_name);

            for (target_details_name, target_details) in targe.iter() {
                let mut banned_target_detail_tree = Node::new((*target_details_name).clone());

                for target_detail in target_details {
                    banned_target_detail_tree.push(
                        target_detail.clone()
                    );
                }

                banned_target_tree.push(banned_target_detail_tree);
            }

            banned_targets_tree.push(banned_target_tree);
        }

        Err(CatructureError::DetectBlacklistBlock(banned_targets_tree.to_string()))
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
    ) -> Target;
}

impl BlockBlacklist for Structure {
    fn detect_blacklist_block<'a>(
            &'a self,
            palette: &'a Vec<PaletteBlock>,
            blacklist: &'a Blacklist
    ) -> Target {
        let mut bucket = Target::new();

        for (index, block) in palette.iter().enumerate() {
            if blacklist.blocks.contains(&block.name) {
                let mut block_positions_tmp = Vec::new();

                self.blocks.iter().for_each(|block_pos| {
                    if block_pos.state as usize == index {
                        block_positions_tmp.push(format!("{} {} {}", block_pos.pos[0], block_pos.pos[1], block_pos.pos[2]));
                    }
                });

                bucket.entry(&block.name).or_default().extend(block_positions_tmp);
            }
        }

        bucket
    }
}
