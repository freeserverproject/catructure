use std::{
    io::Read, path::Path,
    fs::File
};

use serde::{Serialize, Deserialize};

use crate::error::{CatructureError, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub container: ContainerSetting,
    pub blacklist: Blacklist
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContainerSetting {
    Allow,
    Empty
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Blacklist {
    pub blocks: Vec<String>
}

impl Config {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Config> {
        let mut config = String::new();
        File::open(path)
            .map_err(CatructureError::FailedOpenConfigFile)?
            .read_to_string(&mut config)
            .map_err(CatructureError::FailedOpenConfigFile)?;

        toml::from_str::<Config>(&config).map_err(CatructureError::FailedDeserializeConfigFile)
    }
}
