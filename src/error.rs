use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("String could not be deserialised")]
    Deserialize(#[from] toml::de::Error),
    #[error("Std IO error")]
    Io(#[from] std::io::Error),
    #[error("Reqwest failed")]
    Reqwest(#[from] reqwest::Error),
    #[error("Some error occurred")]
    Other(String),
}
