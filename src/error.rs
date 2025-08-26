use thiserror::Error;

#[derive(Error, Debug)]
pub enum IndexerError {
    #[error("Carbon parsing error: {0}")]
    Parsing(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Network error: {0}")]
    Network(String),
}
