use std::{
    collections::HashMap, fs::File, path::Path,
    io::Read, fmt::Display
};

use flate2::read::GzDecoder;
use fastnbt::Value;
use serde::{Serialize, Deserialize};

use crate::error::{Result, CatructureError};

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

impl Structure {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Structure> {
        let file = File::open(path)
            .map_err(CatructureError::FailedReadNBTFile)?;
        let mut decoder = GzDecoder::new(file);
        let mut structure = vec![];
        decoder.read_to_end(&mut structure).map_err(CatructureError::FailedDecodeNBTFile)?;

        fastnbt::from_bytes::<Structure>(&structure)
            .map_err(CatructureError::FailedDeserializeNBTFile)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PaletteBlock {
    pub name: String,
    pub properties: Option<HashMap<String, String>>
}

impl Display for PaletteBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(properties) = &self.properties {
            if properties.len() == 0 { return write!(f, "{}", self.name.clone()); };
            let mut properties = properties
                .iter()
                .collect::<Vec<_>>();

            properties.sort_by(|(key_a, _), (key_b, _)| key_a.cmp(key_b)); 

            let properties = properties
                .iter()
                .map(|(key, value)| format!("{}={}", key, value))
                .collect::<Vec<_>>()
                .join(",");

            write!(
                f,
                "{}[{}]",
                self.name,
                properties
            )
        } else {
            write!(f, "{}", self.name)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlockNBT {
    items: Option<Vec<HashMap<String, Value>>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockPosition {
    pub state: i32,
    pub pos: Vec<i32>,
    pub nbt: Option<BlockNBT>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    pub pos: (f64, f64, f64),
    #[serde(rename = "blockPos")]
    pub block_pos: (i32, i32, i32),
    pub nbt: EntityNBT
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityNBT {
    pub id: String
}
