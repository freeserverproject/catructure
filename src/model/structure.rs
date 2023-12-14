use std::collections::HashMap;

use fastnbt::Value;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Structure {
    #[serde(rename = "DataVersion")]
    pub data_version: i32,
    pub size: Vec<i32>,
    pub palette: Option<Vec<PaletteBlock>>,
    pub palettes: Option<Vec<Vec<PaletteBlock>>>,
    pub blocks: Vec<Block>,
    pub entities: Vec<Entity>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PaletteBlock {
    pub name: String,
    pub properties: Option<HashMap<String, String>>
}

pub type NBT = HashMap<String, Value>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub state: i32,
    pub pos: Vec<i32>,
    pub nbt: Option<NBT>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entity {
    pub pos: Vec<i32>,
    #[serde(rename = "blockPos")]
    pub block_pos: Vec<i32>,
    pub nbt: Option<NBT>
}
