use thiserror::Error;

pub type Result<T> = std::result::Result<T, CatructureError>;

#[derive(Debug, Error)]
pub enum CatructureError {
    #[error("Failed open config file:\n    Caused by {0}")]
    FailedOpenConfigFile(
        #[source] std::io::Error
    ),
    #[error("Failed read config file:\n    Caused by {0}")]
    FailedReadConfigFile(
        #[source] toml::de::Error
    ),
    #[error("Detect blacklist block.\n{0}")]
    DetectBlacklistBlock(String)
}