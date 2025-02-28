use std::{fs::read_to_string, path::Path};

use serde::{Deserialize, Serialize};

use crate::{error::Error, metric::Metric, probe::ProbeInput};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub(crate) struct MetricNew {
    #[serde(rename = "probe")]
    probes: Vec<ProbeInput>,
}

impl MetricNew {
    pub(crate) fn from_file(filepath: &Path) -> Result<Self, Error> {
        let content = read_to_string(filepath)?;
        Self::from_str(&content)
    }

    pub(crate) fn from_str(str: &str) -> Result<Self, Error> {
        let metric: MetricNew = toml::from_str(str)?;
        if metric.probes.is_empty() {
            Err(Error::Other(
                "Metric needs to contain at least one probe".to_string(),
            ))
        } else {
            Ok(metric)
        }
    }
}

impl std::fmt::Display for MetricNew {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match toml::to_string(self) {
            Ok(toml_str) => write!(f, "{}", toml_str),
            Err(err) => write!(f, "Error serializing to TOML: {}", err),
        }
    }
}

impl Metric {
    pub(crate) fn from_file(filepath: &Path) -> Result<Self, Error> {
        let content = read_to_string(filepath)?;
        Self::from_str(&content)
    }

    pub(crate) fn from_str(str: &str) -> Result<Self, Error> {
        let metric: Metric = toml::from_str(str)?;
        if metric.contains_only_none() {
            Err(Error::Other(
                "Metric needs to contain at least one probe".to_string(),
            ))
        } else {
            Ok(metric)
        }
    }

    fn contains_only_none(&self) -> bool {
        let toml_str = match toml::ser::to_string(self) {
            Ok(str) => str,
            Err(_) => return false,
        };
        toml_str.trim().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{remove_file, write};

    use tempfile::NamedTempFile;

    use crate::{
        metric::{EXAMPLE_METRIC, EXAMPLE_METRIC_STR},
        probe_name::ProbeName,
    };

    use super::*;

    #[test]
    fn metric_can_be_read_from_file() {
        let tempfile = NamedTempFile::new().unwrap();
        let filepath = tempfile.path();
        write(filepath, EXAMPLE_METRIC_STR).unwrap();
        let result = MetricNew::from_file(filepath);
        assert!(result.is_ok());

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
weight = 0.2
    "#;
        let metric = MetricNew::from_str(METRIC_WITH_ZERO_STR).expect("Failed to parse metric!");
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

        let result = MetricNew::from_str(METRIC);
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
"#;

        let expected_metric = MetricNew {
            probes: vec![
                ProbeInput {
                    name: ProbeName::archived,
                    weight: -1.,
                },
                ProbeInput {
                    name: ProbeName::codeApproved,
                    weight: 1.,
                },
            ],
        };
        let serialized = toml::to_string(&expected_metric).unwrap();

        assert_eq!(serialized.trim(), EXPECTED_SERIALIZED.trim());

        let metric = toml::from_str::<MetricNew>(&serialized).unwrap();
        assert_eq!(metric, expected_metric);
    }

    #[test]
    fn completely_empty_metric_is_not_ok() {
        assert!(MetricNew::from_str("").is_err());
    }

    #[test]
    fn all_weights_zero_is_treated_as_empty() {
        static METRIC_WITH_ZERO_STR: &str = r#"
[[probe]]
name = "archived"
weight = 0.0
        "#;
        let metric = MetricNew::from_str(METRIC_WITH_ZERO_STR);
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
