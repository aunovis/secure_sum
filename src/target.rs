use std::{fmt::Display, path::PathBuf};

use crate::{
    ecosystem::{parse, DepFile, Ecosystem},
    error::Error,
    url::Url,
};

pub(crate) enum Target {
    Url(Url),
    DepFile(PathBuf, Box<dyn DepFile>),
}

#[derive(PartialEq, Eq)]
pub(crate) enum SingleTarget {
    Package(String, Ecosystem),
    Url(Url),
}

impl Target {
    pub(crate) fn parse(path: String) -> Result<Self, Error> {
        if is_url(&path) {
            return Ok(Self::Url(path.into()));
        }
        let depfile_path = PathBuf::from(&path);
        let depfile = parse(&depfile_path);
        if let Ok(depfile) = depfile {
            return Ok(Self::DepFile(depfile_path, depfile));
        }
        let message = format!("Unable to understand {path}");
        Err(Error::Input(message))
    }
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Target::Url(url) => write!(f, "URL: {url}"),
            Target::DepFile(path, _) => write!(f, "Cargo/Rust: {}", path.display()),
        }
    }
}

impl SingleTarget {
    pub(crate) fn to_scorecard_arg(&self) -> Result<String, Error> {
        match self {
            SingleTarget::Package(package, ecosystem) => ecosystem.dep_to_scorecard_arg(package),
            SingleTarget::Url(url) => Ok(format!("--repo={url}")),
        }
    }
}

impl Display for SingleTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SingleTarget::Package(package, ecosystem) => {
                write!(f, "{} package {package}", ecosystem.as_str())
            }
            SingleTarget::Url(url) => write!(f, "URL: {url}"),
        }
    }
}

fn is_url(str: &str) -> bool {
    str.starts_with("https://") || str.starts_with("http://")
}

pub(crate) fn collect_single_targets(targets: Vec<Target>) -> Vec<SingleTarget> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ecosystem::parse_str_as_depfile;

    #[test]
    fn protocols_mark_urls() {
        assert!(is_url("https://quettapano"));
        assert!(is_url("http://andolama/mirquet"));
        assert!(!is_url("cimrinora/arquenie"));
    }

    #[test]
    fn collect_single_targets_combines_dependencies() {
        let file_1 =
            parse_str_as_depfile(r#"{"dependencies": {"@xenova/transformers": "^2.17.1"}}"#);
        let file_2 = parse_str_as_depfile(r#"{"dependencies": {"handlebars": "^4.7.8"}}"#);
        let targets = vec![
            Target::DepFile(PathBuf::new(), file_1),
            Target::DepFile(PathBuf::new(), file_2),
        ];

        let single_targets = collect_single_targets(targets);

        assert_eq!(single_targets.len(), 2);
        assert!(single_targets.contains(&SingleTarget::Package(
            "@xenova/transformers".to_string(),
            Ecosystem::NodeJs
        )));
        assert!(single_targets.contains(&SingleTarget::Package(
            "handlebars".to_string(),
            Ecosystem::NodeJs
        )));
    }

    #[test]
    fn collect_single_targets_can_mix_and_match_depfiles_and_urls() {
        let file = parse_str_as_depfile(r#"{"dependencies": {"handlebars": "^4.7.8"}}"#);
        let url = "https://github/somethingsomething";
        let targets = vec![
            Target::DepFile(PathBuf::new(), file),
            Target::Url(url.into()),
        ];

        let single_targets = collect_single_targets(targets);

        assert_eq!(single_targets.len(), 2);
        assert!(single_targets.contains(&SingleTarget::Package(
            "handlebars".to_string(),
            Ecosystem::NodeJs
        )));
        assert!(single_targets.contains(&SingleTarget::Url(url.into())));
    }

    #[test]
    fn collect_single_targets_dedups_the_output() {
        let file_1 =
            parse_str_as_depfile(r#"{"dependencies": {"@xenova/transformers": "^4.7.8"}}"#);
        let file_2 = parse_str_as_depfile(r#"{"dependencies": {"handlebars": "^4.7.8"}}"#);
        let file_3 = parse_str_as_depfile(r#"{"dependencies": {"handlebars": "^4.7.8"}}"#);
        let targets = vec![
            Target::DepFile(PathBuf::new(), file_1),
            Target::DepFile(PathBuf::new(), file_2),
            Target::DepFile(PathBuf::new(), file_3),
        ];

        let single_targets = collect_single_targets(targets);

        assert_eq!(single_targets.len(), 2);
        assert!(single_targets.contains(&SingleTarget::Package(
            "@xenova/transformers".to_string(),
            Ecosystem::NodeJs
        )));
        assert!(single_targets.contains(&SingleTarget::Package(
            "handlebars".to_string(),
            Ecosystem::NodeJs
        )));
    }
}
