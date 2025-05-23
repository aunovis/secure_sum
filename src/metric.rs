use std::{
    fs::{self, read_to_string},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{error::Error, filesystem::data_dir, probe::ProbeInput, probe_name::ProbeName};

static DEFAULT_METRIC_URL: &str =
    "https://raw.githubusercontent.com/aunovis/secure_sum/refs/heads/main/default_metric.toml";

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub(crate) struct Metric {
    #[serde(default)]
    pub(crate) error_threshold: Option<f32>,
    #[serde(default)]
    pub(crate) warn_threshold: Option<f32>,
    #[serde(rename = "probe")]
    pub(crate) probes: Vec<ProbeInput>,
}

fn default_metric_file_path() -> Result<PathBuf, Error> {
    Ok(data_dir()?.join("default_metric.toml"))
}

fn ensure_default_metric_file() -> Result<PathBuf, Error> {
    let path = default_metric_file_path()?;
    if path.exists() {
        return Ok(path);
    }
    let dir = data_dir()?;
    fs::create_dir_all(&dir)?;
    log::info!("Downloading Default Metric from {DEFAULT_METRIC_URL}.");
    let response = reqwest::blocking::get(DEFAULT_METRIC_URL)?;
    let metric_text = response.text()?;
    fs::write(&path, metric_text)?;
    log::info!("Stored default metric file under \"{}\".", path.display());
    Ok(path)
}

impl Metric {
    pub(crate) fn new(filepath: Option<&Path>) -> Result<Self, Error> {
        match filepath {
            Some(path) => Self::from_file(path),
            None => {
                let path = ensure_default_metric_file()?;
                Self::from_file(&path)
            }
        }
    }

    fn from_file(filepath: &Path) -> Result<Self, Error> {
        log::debug!("Reading metric file from \"{}\".", filepath.display());
        let content = read_to_string(filepath)?;
        Self::from_str(&content)
    }

    pub(crate) fn from_str(str: &str) -> Result<Self, Error> {
        let mut metric: Metric = toml::from_str(str)?;
        metric.probes.retain(|p| !p.is_zeroweight());
        metric.probes.retain(|p| !p.is_zero_times());
        log::debug!("Parsed metric:\n{metric}");
        metric.consistency_check()?;
        Ok(metric)
    }

    fn consistency_check(&self) -> Result<(), Error> {
        if self.probes.is_empty() {
            return Err(Error::Other(
                "Metric needs to contain at least one probe".to_string(),
            ));
        }

        let probe_names = self.probe_names();
        let duplicates: Vec<_> = probe_names
            .windows(2)
            .filter_map(|names| {
                if names[0] == names[1] {
                    Some(names[0])
                } else {
                    None
                }
            })
            .collect();
        if !duplicates.is_empty() {
            let duplicates = duplicates
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            let message = format!("Metric contains duplicate probes: {duplicates}");
            return Err(Error::Other(message));
        }

        Ok(())
    }

    pub(crate) fn probe_names(&self) -> Vec<ProbeName> {
        let mut names: Vec<_> = self.probes.iter().map(|p| p.name).collect();
        names.sort();
        names
    }
}

impl std::fmt::Display for Metric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match toml::to_string(self) {
            Ok(toml_str) => write!(f, "{}", toml_str),
            Err(err) => write!(f, "Error serializing to TOML: {}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{remove_file, write};

    use serial_test::serial;
    use tempfile::NamedTempFile;

    use crate::probe_name::ProbeName;

    use super::*;

    #[test]
    fn metric_can_be_read_from_file() {
        static EXAMPLE_METRIC_STR: &str = r#"
error_threshold = 4
warn_threshold = 5

[[probe]]
name = "archived"
weight = -1.0

[[probe]]
name = "codeApproved"
weight = 1.0
max_times = 12
        "#;
        let expected = Metric {
            error_threshold: Some(4.),
            warn_threshold: Some(5.),
            probes: vec![
                ProbeInput {
                    name: ProbeName::archived,
                    weight: -1.,
                    max_times: None,
                },
                ProbeInput {
                    name: ProbeName::codeApproved,
                    weight: 1.,
                    max_times: Some(12),
                },
            ],
        };

        let tempfile = NamedTempFile::new().unwrap();
        let filepath = tempfile.path();
        write(filepath, EXAMPLE_METRIC_STR).unwrap();
        let metric = Metric::from_file(filepath).unwrap();
        assert_eq!(metric, expected);

        remove_file(filepath).ok();
    }

    #[test]
    fn weight_of_zero_is_treated_as_none() {
        static METRIC_WITH_ZERO_STR: &str = r#"
[[probe]]
name = "archived"
weight = 0.0

[[probe]]
name = "blocksDeleteOnBranches"
weight = 1
        "#;
        let metric = Metric::from_str(METRIC_WITH_ZERO_STR).expect("Failed to parse metric!");
        assert_eq!(metric.probes.len(), 1);
    }

    #[test]
    fn max_times_of_zero_is_treated_as_none() {
        static METRIC_WITH_ZERO_STR: &str = r#"
[[probe]]
name = "archived"
weight = 1
max_times = 0

[[probe]]
name = "blocksDeleteOnBranches"
weight = 1
        "#;
        let metric = Metric::from_str(METRIC_WITH_ZERO_STR).expect("Failed to parse metric!");
        assert_eq!(metric.probes.len(), 1);
    }

    #[test]
    fn probe_duplications_are_error() {
        static METRIC: &str = r#"
[[probe]]
name = "archived"
weight = -1.0

[[probe]]
name = "archived"
weight = 1.0
        "#;

        let result = Metric::from_str(METRIC);
        assert!(result.is_err())
    }

    #[test]
    fn metric_serialization_roundtrip() {
        static EXPECTED_SERIALIZED: &str = r#"
warn_threshold = 5.0

[[probe]]
name = "archived"
weight = -1.0

[[probe]]
name = "codeApproved"
weight = 1.0
max_times = 12
        "#;

        let expected_metric = Metric {
            error_threshold: None,
            warn_threshold: Some(5.),
            probes: vec![
                ProbeInput {
                    name: ProbeName::archived,
                    weight: -1.,
                    max_times: None,
                },
                ProbeInput {
                    name: ProbeName::codeApproved,
                    weight: 1.,
                    max_times: Some(12),
                },
            ],
        };
        let serialized = toml::to_string(&expected_metric).unwrap();

        assert_eq!(serialized.trim(), EXPECTED_SERIALIZED.trim());

        let metric = toml::from_str::<Metric>(&serialized).unwrap();
        assert_eq!(metric, expected_metric);
    }

    #[test]
    fn completely_empty_metric_is_not_ok() {
        assert!(Metric::from_str("").is_err());
    }

    #[test]
    fn all_weights_zero_is_treated_as_empty() {
        static METRIC_WITH_ZERO_STR: &str = r#"
[[probe]]
name = "archived"
weight = 0.0
        "#;
        let metric = Metric::from_str(METRIC_WITH_ZERO_STR);
        assert!(metric.is_err());
    }

    #[test]
    fn unknown_probe_is_error() {
        static WEIRD_METRIC: &str = r#"
[[probe]]
name = "archived"
weight = -1.0

[[probe]]
name = "definetelyNotAFieldThatSecureSumWouldExpectGivenThePresentStateOfTheEconomy"
weight = 1.0
        "#;
        assert!(Metric::from_str(WEIRD_METRIC).is_err());
    }

    #[test]
    #[serial]
    fn ensure_metric_file_ensures_metric_file() {
        let path = default_metric_file_path().unwrap();
        fs::remove_file(&path).ok();

        assert!(ensure_default_metric_file().is_ok());
        assert!(path.exists());
        assert!(ensure_default_metric_file().is_ok());
        assert!(path.exists());
    }

    #[test]
    #[serial]
    fn default_metric_file_can_be_read() {
        let path = ensure_default_metric_file().unwrap();
        let result = Metric::from_file(&path);
        assert!(result.is_ok(), "{}", result.unwrap_err())
    }
}
