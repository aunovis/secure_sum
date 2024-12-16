use std::path::PathBuf;

use etcetera::{choose_app_strategy, AppStrategy, AppStrategyArgs};

use crate::error::Error;

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
    Ok(data_dir()?.join("scorecard.exe"))
}

fn ensure_scorecard_binary() -> Result<PathBuf, Error> {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

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
