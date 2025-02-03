use crate::Error;

use super::rust::repo_url;

pub(crate) enum Ecosystem {
    NodeJs,
    Rust,
}

impl Ecosystem {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Ecosystem::NodeJs => "Node.js",
            Ecosystem::Rust => "rust",
        }
    }

    pub(crate) fn dep_to_scorecard_arg(&self, dep: &str) -> Result<String, Error> {
        match self {
            Ecosystem::NodeJs => Ok(format!("--npm={dep}")),
            Ecosystem::Rust => repo_url(dep).map(|url| format!("--repo={url}")),
        }
    }
}
