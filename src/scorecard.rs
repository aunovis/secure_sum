use std::path::PathBuf;

use crate::error::Error;

fn scorecard_path() -> PathBuf {
    todo!()
}

fn ensure_scorecard_binary() -> Result<PathBuf, Error> {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    fn scorecard_path_contains_aunovis_and_scorecard() {
        let path = scorecard_path().to_string_lossy().to_lowercase();
        assert!(path.contains("aunovis"));
        assert!(path.contains("scorecard"))
    }

    fn scorecard_binary_exists_after_ensure_scorecard_binary_call() {
        ensure_scorecard_binary().expect("Ensuring scorecard binary failed");
        let path = scorecard_path();
        assert!(fs::exists(path).unwrap());
    }
}
