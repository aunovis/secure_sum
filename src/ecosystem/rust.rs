use std::path::Path;

use crate::error::Error;

use super::{depfile::DepFile, Ecosystem};

pub(super) struct CargoToml {}

impl CargoToml {
    pub(super) fn parse(file: &Path) -> Result<Self, Error> {
        todo!()
    }
}

impl DepFile for CargoToml {
    fn ecosystem(&self) -> super::Ecosystem {
        Ecosystem::Rust
    }

    fn first_level_deps(&self) -> Vec<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn cargo_toml_can_be_parsed() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
        assert!(path.exists());
        let result = CargoToml::parse(&path);
        assert!(result.is_ok(), "{}", result.err().unwrap());
        let cargo_toml = result.unwrap();
        assert!(cargo_toml.first_level_deps().contains(&"serde".to_string()));
    }
}
