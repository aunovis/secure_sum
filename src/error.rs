use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("string could not be parsed")]
    Deserialize(#[from] toml::de::Error),
}
