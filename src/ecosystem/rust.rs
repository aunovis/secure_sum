use std::{collections::HashMap, fs, path::Path};

use reqwest::blocking::Client;
use serde::Deserialize;

use crate::{error::Error, target::SingleTarget, url::Url};

use super::{depfile::DepFile, Ecosystem};

#[derive(Debug, Deserialize)]
pub(super) struct CargoToml {
    #[serde(default)]
    dependencies: HashMap<String, toml::Value>,
}

impl CargoToml {
    pub(super) fn parse(file: &Path) -> Result<Self, Error> {
        let contents = fs::read_to_string(file)?;
        Self::parse_str(&contents)
    }

    fn parse_str(contents: &str) -> Result<Self, Error> {
        let depfile = toml::from_str(contents)?;
        Ok(depfile)
    }
}

impl DepFile for CargoToml {
    fn ecosystem(&self) -> super::Ecosystem {
        Ecosystem::Rust
    }

    fn first_level_deps(&self) -> Vec<SingleTarget> {
        self.dependencies
            .keys()
            .map(|dep| SingleTarget::Package(dep.to_owned(), self.ecosystem()))
            .collect()
    }
}

#[derive(Deserialize)]
struct CrateResponse {
    #[serde(rename = "crate")]
    crate_: Crate,
}

#[derive(Deserialize)]
struct Crate {
    repository: Option<String>,
}

pub(super) fn repo_url(crate_name: &str) -> Result<Url, Error> {
    let url = format!("https://crates.io/api/v1/crates/{}", crate_name);
    let client = Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "secure_sum (info@aunovis.de)")
        .send()?
        .text()?;

    let crate_response: CrateResponse = serde_json::from_str(&response)?;
    match crate_response.crate_.repository {
        Some(repo) => Ok(repo.into()),
        None => {
            let message = format!("Could not obtain repo for crate {}", crate_name);
            Err(Error::Other(message))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn empty_depfile_can_be_parsed() {
        let result = CargoToml::parse_str("");
        assert!(result.is_ok(), "{}", result.err().unwrap());
        let depfile = result.unwrap();
        assert!(depfile.dependencies.is_empty());
    }

    #[test]
    fn small_depfile_can_be_parsed() {
        let content = r#"
        [dependencies]
        serde = "1.0"
        toml = { version = "0.5", features = ["derive"] }
    "#;
        let result = CargoToml::parse_str(&content);
        assert!(result.is_ok(), "{}", result.err().unwrap());
        let depfile = result.unwrap();
        assert_eq!(depfile.dependencies.len(), 2);
        assert!(depfile.dependencies.contains_key("serde"));
        assert!(depfile.dependencies.contains_key("toml"));
    }

    #[test]
    fn secure_sum_cargo_toml_file_can_be_parsed() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
        assert!(path.exists());
        let result = CargoToml::parse(&path);
        assert!(result.is_ok(), "{}", result.err().unwrap());
        let cargo_toml = result.unwrap();
        assert!(cargo_toml.dependencies.contains_key("serde"));
    }

    #[test]
    fn crate_repo_url_can_be_obtained() {
        let crate_name = "serde";
        let result = repo_url(crate_name);
        assert!(result.is_ok(), "{}", result.err().unwrap());
        let repo = result.unwrap();
        assert_eq!(repo.0, "https://github.com/serde-rs/serde");
    }
}
