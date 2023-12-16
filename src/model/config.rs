use serde::{Serialize, Deserialize};

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
