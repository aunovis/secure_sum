use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{error::Error, probe::ProbeInput, probe_name::ProbeName};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub(crate) struct Metric {
    #[serde(rename = "probe")]
    pub(crate) probes: Vec<ProbeInput>,
}

fn default_metric_file_path() -> Result<PathBuf, Error> {
    todo!()
}

fn ensure_default_metric_file() -> Result<(), Error> {
    todo!()
}

impl Metric {
    pub(crate) fn new(filepath: Option<&Path>) -> Result<Self, Error> {
        match filepath {
            Some(path) => Self::from_file(path),
            None => {
                ensure_default_metric_file()?;
                let path = default_metric_file_path()?;
                Self::from_file(&path)
            }
        }
    }

    fn from_file(filepath: &Path) -> Result<Self, Error> {
        let content = read_to_string(filepath)?;
        Self::from_str(&content)
    }

    pub(crate) fn from_str(str: &str) -> Result<Self, Error> {
        let mut metric: Metric = toml::from_str(str)?;
        metric.probes.retain(|p| !p.is_zeroweight());
        metric.probes.retain(|p| !p.is_zero_times());

        if metric.probes.is_empty() {
            return Err(Error::Other(
                "Metric needs to contain at least one probe".to_string(),
            ));
        }

        let probe_names = metric.probe_names();
        let duplicates: Vec<_> = probe_names
            .windows(2)
            .filter_map(|window| {
                if window[0] == window[1] {
                    Some(window[0])
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

        Ok(metric)
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

    use tempfile::NamedTempFile;

    use crate::probe_name::ProbeName;

    use super::*;

    #[test]
    fn metric_can_be_read_from_file() {
        static EXAMPLE_METRIC_STR: &str = r#"
[[probe]]
name = "archived"
weight = -1.0

[[probe]]
name = "codeApproved"
weight = 1.0
max_times = 12
        "#;
        let expected = Metric {
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
[[probe]]
name = "archived"
weight = -1.0

[[probe]]
name = "codeApproved"
weight = 1.0
max_times = 12
        "#;

        let expected_metric = Metric {
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
}
