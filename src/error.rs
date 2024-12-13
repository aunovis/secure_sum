use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("string could not be deserialised")]
    Deserialize(#[from] toml::de::Error),
    #[error("Std IO error")]
    Io(#[from] std::io::Error),
}
