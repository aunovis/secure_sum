use std::{collections::HashMap, fs, path::Path};

use serde::Deserialize;

use crate::{target::SingleTarget, Error};

use super::{DepFile, Ecosystem};

#[derive(Debug, Deserialize)]
pub(super) struct PackageJson {
    #[serde(default)]
    dependencies: HashMap<String, String>,
}

impl DepFile for PackageJson {
    fn ecosystem(&self) -> super::Ecosystem {
        Ecosystem::NodeJs
    }

    fn first_level_deps(&self) -> Vec<SingleTarget> {
        self.dependencies
            .keys()
            .map(|dep| SingleTarget::Package(dep.to_owned(), self.ecosystem()))
            .collect()
    }
}

impl PackageJson {
    pub(super) fn parse(file: &Path) -> Result<Self, Error> {
        let contents = fs::read_to_string(file)?;
        Self::parse_str(&contents)
    }

    fn parse_str(contents: &str) -> Result<Self, Error> {
        let depfile = serde_json::from_str(contents)?;
        Ok(depfile)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_depfile_can_be_parsed() {
        let result = PackageJson::parse_str("{}");
        assert!(result.is_ok(), "{}", result.err().unwrap());
        let depfile = result.unwrap();
        assert!(depfile.dependencies.is_empty());
    }

    #[test]
    fn small_depfile_can_be_parsed() {
        let content = r#"
    {
        "dependencies": {
            "@xenova/transformers": "^2.17.1",
            "handlebars": "^4.7.8"
        }
    }
    "#;
        let result = PackageJson::parse_str(&content);
        assert!(result.is_ok(), "{}", result.err().unwrap());
        let depfile = result.unwrap();
        assert_eq!(depfile.dependencies.len(), 2);
        assert!(depfile.dependencies.contains_key("@xenova/transformers"));
        assert!(depfile.dependencies.contains_key("handlebars"));
    }
}
