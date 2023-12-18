use thiserror::Error;

pub type Result<T> = std::result::Result<T, CatructureError>;

#[derive(Debug, Error)]
pub enum CatructureError {
    #[error("Failed to open config file:\n  Caused by: {0}")]
    FailedOpenConfigFile(#[source] std::io::Error),
    
    #[error("Failed to read config file:\n  Caused by: {0}")]
    FailedDeserializeConfigFile(#[source] toml::de::Error),

    #[error("Detect blacklist block.\n{0}")]
    DetectBlacklistBlock(String),

    #[error("Failed to open nbt file.\n  Caused by: {0}")]
    FailedReadNBTFile(#[source] std::io::Error),

    #[error("Failed to decode nbt file in gz.\n  Caused by: {0}")]
    FailedDecodeNBTFile(#[source] std::io::Error),

    #[error("Failed to deserialize nbt file.\n  Caused by: {0}")]
    FailedDeserializeNBTFile(#[source] fastnbt::error::Error)
}
