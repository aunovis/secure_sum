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
