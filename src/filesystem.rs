use std::path::PathBuf;

use etcetera::{AppStrategy, AppStrategyArgs, choose_app_strategy};

use crate::error::Error;

#[cfg(target_os = "macos")]
pub(crate) static OS_STR: &str = "darwin";
#[cfg(target_os = "linux")]
pub(crate) static OS_STR: &str = "linux";
#[cfg(target_os = "windows")]
pub(crate) static OS_STR: &str = "windows";

/// target_arch config is not recognised on all OSs.
/// We therefore only check for "arm or not arm".-
#[cfg(target_arch = "arm")]
pub(crate) static ARCH_STR: &str = "arm64";
#[cfg(not(target_arch = "arm"))]
pub(crate) static ARCH_STR: &str = "amd64";

pub(crate) fn data_dir() -> Result<PathBuf, Error> {
    let app_strategy_args = AppStrategyArgs {
        top_level_domain: "de".to_string(),
        author: "aunovis".to_string(),
        app_name: "aunovis_secure_sum".to_string(),
    };
    let data_dir = choose_app_strategy(app_strategy_args)
        .map_err(|e| Error::Other(e.to_string()))?
        .data_dir();
    Ok(data_dir)
}
