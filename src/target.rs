use std::{fmt::Display, path::PathBuf};

use crate::{
    ecosystem::{try_parse_all_ecosystems, DepFile},
    error::Error,
};

pub(crate) enum Target {
    Url(String),
    DepFile(PathBuf, Box<dyn DepFile>),
}

impl Target {
    pub(crate) fn parse(path: String) -> Result<Self, Error> {
        if is_url(&path) {
            return Ok(Self::Url(path));
        }
        let depfile_path = PathBuf::from(&path);
        let depfile = try_parse_all_ecosystems(&depfile_path);
        if let Ok(depfile) = depfile {
            return Ok(Self::DepFile(depfile_path, depfile));
        }
        let message = format!("Unable to understand {path}");
        Err(Error::Input(message))
    }
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Target::Url(url) => write!(f, "URL: {url}"),
            Target::DepFile(path, _) => write!(f, "Cargo/Rust: {}", path.display()),
        }
    }
}

fn is_url(str: &str) -> bool {
    str.starts_with("https://") || str.starts_with("http://")
}

fn is_cargo_toml(str: &str) -> bool {
    str.to_lowercase().ends_with("cargo.toml")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protocols_mark_urls() {
        assert!(is_url("https://quettapano"));
        assert!(is_url("http://andolama/mirquet"));
        assert!(!is_url("cimrinora/arquenie"));
    }
}
