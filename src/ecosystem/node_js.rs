use serde::Deserialize;

use crate::target::SingleTarget;

use super::{DepFile, Ecosystem};

#[derive(Debug, Deserialize)]
pub(super) struct PackageJson {}

impl DepFile for PackageJson {
    fn ecosystem(&self) -> super::Ecosystem {
        Ecosystem::NodeJs
    }

    fn first_level_deps(&self) -> Vec<SingleTarget> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn empty_depfile_can_be_parsed() {
        // let result = PackageJson::parse_str("");
        // assert!(result.is_ok(), "{}", result.err().unwrap());
        // let depfile = result.unwrap();
        // assert!(depfile.dependencies.is_empty());
    }
}
