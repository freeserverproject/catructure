use std::{
    io::Read, path::Path,
    fs::File
};

use serde::{Serialize, Deserialize};

use crate::error::{CatructureError, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub container: ContainerSetting,
    pub entity: EntitySetting,
    pub blacklist: Blacklist
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Config> {
        let mut config = String::new();
        File::open(path)
            .map_err(CatructureError::FailedOpenConfigFile)?
            .read_to_string(&mut config)
            .map_err(CatructureError::FailedOpenConfigFile)?;

        toml::from_str::<Config>(&config).map_err(CatructureError::FailedDeserializeConfigFile)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContainerSetting {
    Allow,
    Deny
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntitySetting {
    Allow,
    Deny
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Blacklist {
    pub blocks: Vec<String>
}
