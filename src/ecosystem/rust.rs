use std::{collections::HashMap, fs, path::Path};

use serde::Deserialize;

use crate::{error::Error, url::Url};

use super::{depfile::DepFile, Ecosystem};

#[derive(Debug, Deserialize)]
pub(super) struct CargoToml {
    #[serde(default)]
    dependencies: HashMap<String, toml::Value>,
}

impl CargoToml {
    pub(super) fn parse(file: &Path) -> Result<Self, Error> {
        let contents = fs::read_to_string(&file)?;
        Self::parse_str(&contents)
    }

    fn parse_str(contents: &str) -> Result<Self, Error> {
        let cargo_toml = toml::from_str(contents)?;
        Ok(cargo_toml)
    }
}

impl DepFile for CargoToml {
    fn ecosystem(&self) -> super::Ecosystem {
        Ecosystem::Rust
    }

    fn first_level_deps(&self) -> Vec<Url> {
        // self.dependencies.keys().cloned().collect()
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn empty_toml_can_be_parsed() {
        let result = CargoToml::parse_str("");
        assert!(result.is_ok(), "{}", result.err().unwrap());
        let cargo_toml = result.unwrap();
        assert!(cargo_toml.first_level_deps().is_empty());
    }

    #[test]
    fn small_toml_can_be_parsed() {
        let content = r#"
        [dependencies]
        serde = "1.0"
        toml = { version = "0.5", features = ["derive"] }
    "#;
        let result = CargoToml::parse_str(&content);
        assert!(result.is_ok(), "{}", result.err().unwrap());
        let cargo_toml = result.unwrap();
        assert_eq!(cargo_toml.first_level_deps().len(), 2);
        assert!(cargo_toml.first_level_deps().contains(&"serde".into()));
        assert!(cargo_toml.first_level_deps().contains(&"toml".into()));
    }

    #[test]
    fn secure_sum_cargo_toml_file_can_be_parsed() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
        assert!(path.exists());
        let result = CargoToml::parse(&path);
        assert!(result.is_ok(), "{}", result.err().unwrap());
        let cargo_toml = result.unwrap();
        assert!(cargo_toml.first_level_deps().contains(&"serde".into()));
    }
}
