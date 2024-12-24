use std::string::FromUtf8Error;

use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("String could not be deserialized: {0}")]
    Deserialize(#[from] toml::de::Error),

    #[error("Failed to interpret bytes as UTF-8: {0}")]
    FromUtf8(#[from] FromUtf8Error),

    #[error("Unrecognized or unsupported input: {0}")]
    Input(String),

    #[error("IO error occurred: {0}")]
    Io(#[from] std::io::Error),

    #[error("Reqwest error occurred: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Scorecard error: {0}")]
    Scorecard(String),

    #[error("An unspecified error occurred: {0}")]
    Other(String),
}
