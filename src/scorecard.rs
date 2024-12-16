use std::{fs, path::PathBuf};

use etcetera::{choose_app_strategy, AppStrategy, AppStrategyArgs};

use crate::error::Error;

static CURRENT_VERSION: &str = "5.0.0";

#[cfg(target_os = "macos")]
static OS_STR: &str = "darwin";
#[cfg(target_os = "linux")]
static OS_STR: &str = "linux";
#[cfg(target_os = "windows")]
static OS_STR: &str = "windows";

#[cfg(target_arch = "x86_64")]
static ARCH_STR: &str = "amd64";
#[cfg(target_arch = "arm")]
static ARCH_STR: &str = "arm64";

fn scorecard_url() -> String {
    format!("https://github.com/ossf/scorecard/releases/download/v{CURRENT_VERSION}/scorecard_{CURRENT_VERSION}_{OS_STR}_{ARCH_STR}.tar.gz")
}

fn data_dir() -> Result<PathBuf, Error> {
    let app_strategy_args = AppStrategyArgs {
        top_level_domain: "de".to_string(),
        author: "aunovis".to_string(),
        app_name: "secure_sum".to_string(),
    };
    let data_dir = choose_app_strategy(app_strategy_args)
        .map_err(|e| Error::Other(e.to_string()))?
        .data_dir();
    Ok(data_dir)
}

fn scorecard_path() -> Result<PathBuf, Error> {
    let executable_name = if cfg!(target_os = "windows") {
        "scorecard.exe"
    } else {
        "scorecard"
    };
    Ok(data_dir()?.join(executable_name))
}

fn ensure_scorecard_binary() -> Result<PathBuf, Error> {
    let path = scorecard_path()?;
    fs::create_dir_all(&path)?;
    todo!()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    fn scorecard_url_exists() {
        todo!()
    }

    fn data_dir_contains_aunovis_string() {
        let path = scorecard_path().unwrap().to_string_lossy().to_lowercase();
        assert!(path.contains("aunovis"));
    }

    fn scorecard_path_contains_scorecard_string() {
        let path = scorecard_path().unwrap().to_string_lossy().to_lowercase();
        assert!(path.contains("scorecard"));
    }

    fn scorecard_binary_exists_after_ensure_scorecard_binary_call() {
        ensure_scorecard_binary().expect("Ensuring scorecard binary failed");
        let path = scorecard_path().unwrap();
        assert!(fs::exists(path).unwrap());
    }
}
