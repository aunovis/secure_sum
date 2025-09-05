use std::{collections::HashMap, fs, path::Path};

use reqwest::blocking::Client;
use serde::Deserialize;

use crate::{
    Error,
    github_token::{USER_AGENT, USER_AGENT_HEADER},
    target::SingleTarget,
    url::Url,
};

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

#[derive(Deserialize)]
struct NpmJsResponse {
    repository: Repository,
}

#[derive(Deserialize)]
struct Repository {
    url: String,
}

pub(super) fn repo_url(package_name: &str) -> Result<Url, Error> {
    let npmjs_url = format!("https://registry.npmjs.org/{package_name}");
    let client = Client::new();
    let response = client
        .get(&npmjs_url)
        .header(USER_AGENT_HEADER, USER_AGENT)
        .send()?
        .text()?;

    let npm_response: NpmJsResponse = serde_json::from_str(&response)?;
    let git_clone_arg = npm_response.repository.url;
    let url = extract_repo_url(&git_clone_arg);
    Ok(url)
}

fn extract_repo_url(git_clone_arg: &str) -> Url {
    let url = git_clone_arg.trim_start_matches("git+");
    let url = url.trim_end_matches(".git");
    url.replace("git@", "https://").into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_depfile_can_be_parsed() {
        let result = PackageJson::parse_str("{}");
        assert!(result.is_ok(), "{}", result.unwrap_err());
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
        assert!(result.is_ok(), "{}", result.unwrap_err());
        let depfile = result.unwrap();
        assert_eq!(depfile.dependencies.len(), 2);
        assert!(depfile.dependencies.contains_key("@xenova/transformers"));
        assert!(depfile.dependencies.contains_key("handlebars"));
    }

    #[test]
    fn npmjs_repo_url_can_be_obtained() {
        let package_name = "handlebars";
        let result = repo_url(package_name);
        assert!(result.is_ok(), "{}", result.unwrap_err());
        let repo = result.unwrap();
        assert_eq!(repo.0, "https://github.com/handlebars-lang/handlebars.js");
    }
}
