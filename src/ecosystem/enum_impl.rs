use crate::Error;

use super::rust::repo_url;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Ecosystem {
    NodeJs,
    NuGet,
    Rust,
}

impl Ecosystem {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Ecosystem::NodeJs => "Node.js",
            Ecosystem::NuGet => "NuGet",
            Ecosystem::Rust => "Rust",
        }
    }

    pub(crate) fn dep_to_scorecard_arg(&self, dep: &str) -> Result<String, Error> {
        match self {
            Ecosystem::NodeJs => Ok(format!("--npm={dep}")),
            Ecosystem::NuGet => Ok(format!("--nuget={dep}")),
            Ecosystem::Rust => repo_url(dep).map(|url| format!("--repo={url}")),
        }
    }
}
