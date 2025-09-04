use std::string::FromUtf8Error;

use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("Error parsing .env file: {0}")]
    Dotenvy(#[from] dotenvy::Error),

    #[error("Failed to interpret bytes as UTF-8: {0}")]
    FromUtf8(#[from] FromUtf8Error),

    #[error("Unrecognized or unsupported input: {0}")]
    Input(String),

    #[error("IO error occurred: {0}")]
    Io(#[from] std::io::Error),

    #[error("FlexiLogger error occurred: {0}")]
    Logger(#[from] flexi_logger::FlexiLoggerError),

    #[error("Reqwest error occurred: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Serde JSON error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("Toml error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("Quick XML error: {0}")]
    QuickXml(#[from] quick_xml::de::DeError),

    #[error("At least one probed repo has a score that is unacceptably low.")]
    ScoreTooLow,

    #[error("The evaluation timed out. Perhaps you have hit a rate limit.")]
    Timeout,

    #[error("An unspecified error occurred: {0}")]
    Other(String),
}
