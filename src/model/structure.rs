use std::collections::HashMap;

use fastnbt::Value;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Structure {
    #[serde(rename = "DataVersion")]
    pub data_version: i32,
    pub size: Vec<i32>,
    pub palette: Option<Vec<PaletteBlock>>,
    pub palettes: Option<Vec<Vec<PaletteBlock>>>,
    pub blocks: Vec<BlockPosition>,
    pub entities: Vec<Entity>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PaletteBlock {
    pub name: String,
    pub properties: Option<HashMap<String, String>>
}

pub type NBT = HashMap<String, Value>;

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockPosition {
    pub state: i32,
    pub pos: Vec<i32>,
    pub nbt: Option<NBT>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    pub pos: Vec<i32>,
    #[serde(rename = "blockPos")]
    pub block_pos: Vec<i32>,
    pub nbt: Option<NBT>
}
