use std::{fs, path::Path};

use serde::Deserialize;

use crate::{target::SingleTarget, Error};

use super::{DepFile, Ecosystem};

#[derive(Debug, Deserialize)]
pub(super) struct PackagesConfig {
    #[serde(default)]
    packages: Vec<Package>,
}

#[derive(Debug, Deserialize)]
struct Package {
    id: String,
}

impl DepFile for PackagesConfig {
    fn ecosystem(&self) -> super::Ecosystem {
        Ecosystem::NuGet
    }

    fn first_level_deps(&self) -> Vec<SingleTarget> {
        self.packages
            .iter()
            .map(|dep| SingleTarget::Package(dep.id.to_owned(), self.ecosystem()))
            .collect()
    }
}

impl PackagesConfig {
    pub(super) fn parse(file: &Path) -> Result<Self, Error> {
        let contents = fs::read_to_string(file)?;
        Self::parse_str(&contents)
    }

    fn parse_str(contents: &str) -> Result<Self, Error> {
        let depfile = quick_xml::de::from_str(contents)?;
        Ok(depfile)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_packages_config_can_be_parsed() {
        let result = PackagesConfig::parse_str("{}");
        assert!(result.is_ok(), "{}", result.err().unwrap());
        let depfile = result.unwrap();
        assert!(depfile.packages.is_empty());
    }

    #[test]
    fn small_packages_config_can_be_parsed() {
        let content = r#"
<?xml version="1.0" encoding="utf-8"?>
<packages>
  <package id="Microsoft.Guardian.Cli" version="0.109.0"/>
</packages>
    "#;
        let result = PackagesConfig::parse_str(&content);
        assert!(result.is_ok(), "{}", result.err().unwrap());
        let depfile = result.unwrap();
        assert_eq!(depfile.packages.len(), 1);
        assert_eq!(depfile.packages[0].id, "Microsoft.Guardian.Cli");
    }
}
