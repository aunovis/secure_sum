use std::path::Path;

use crate::{
    ecosystem::{node_js::PackageJson, rust::CargoToml},
    error::Error,
    target::SingleTarget,
};

use super::Ecosystem;

pub(crate) trait DepFile {
    fn ecosystem(&self) -> Ecosystem;
    fn first_level_deps(&self) -> Vec<SingleTarget>;
}

pub(crate) fn parse(file: &Path) -> Result<Box<dyn DepFile>, Error> {
    let dep_file = try_parse_all_ecosystems(file)?;
    log::debug!(
        "Successfully parsed dependency file for ecosystem {}",
        dep_file.ecosystem().as_str()
    );
    Ok(dep_file)
}

fn try_parse_all_ecosystems(file: &Path) -> Result<Box<dyn DepFile>, Error> {
    if let Ok(cargo_toml) = CargoToml::parse(file) {
        return Ok(Box::new(cargo_toml));
    }
    if let Ok(package_json) = PackageJson::parse(file) {
        return Ok(Box::new(package_json));
    }
    static QUESTION: &str = "Is the ecosystem perhaps not yet supported?";
    static CTA: &str = "In that case, feel free to open an issue on GitHub:";
    static LINK: &str = "https://github.com/aunovis/secure_sum/issues";
    let message = format!(
        "Could not parse {} as a dependency file.\n{QUESTION}\n{CTA}\n{LINK}",
        file.display()
    );
    Err(Error::Other(message))
}
