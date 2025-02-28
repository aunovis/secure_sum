use std::{fs::read_to_string, path::Path};

use serde::{Deserialize, Serialize};

use crate::{error::Error, metric::Metric, probe::ProbeInput};

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct MetricNew {
    #[serde(rename = "probe")]
    probes: Vec<ProbeInput>,
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
        let metric = Metric::from_file(filepath).expect("Failed to read from file!");
        assert_eq!(metric, EXAMPLE_METRIC);

        let _ = remove_file(filepath);
    }

    #[test]
    fn probes_are_optional() {
        static ONLY_ONE_PROBE: &str = "archived = 0.1";
        let metric = Metric::from_str(ONLY_ONE_PROBE).expect("Failed to parse metric!");
        assert_eq!(metric.archived, Some(0.1));
        assert_eq!(metric.blocksDeleteOnBranches, None);

        static ONLY_ONE_OTHER_PROBE: &str = "blocksDeleteOnBranches = 0.2";
        let metric = Metric::from_str(ONLY_ONE_OTHER_PROBE).expect("Failed to parse metric!");
        assert_eq!(metric.archived, None);
        assert_eq!(metric.blocksDeleteOnBranches, Some(0.2));
    }

    #[test]
    fn weight_of_zero_is_treated_as_none() {
        static METRIC_WITH_ZERO_STR: &str = r#"
        archived = 0.0
        blocksDeleteOnBranches = 0.2
    "#;
        let metric = Metric::from_str(METRIC_WITH_ZERO_STR).expect("Failed to parse metric!");
        assert_eq!(metric.archived, None);
    }

    #[test]
    fn simple_roundtrip() {
        static SOME_METRIC: &str = "blocksDeleteOnBranches = 0.2";
        let metric = Metric::from_str(SOME_METRIC).unwrap();
        let metric_str = format!("{metric}");
        // Due to floating point errors the resulting serialized string will not be exactly equal to the input.
        assert!(
            metric_str.starts_with(SOME_METRIC),
            "{SOME_METRIC}\n{metric_str}"
        );
    }

    #[test]
    fn completely_empty_metric_is_not_ok() {
        assert!(Metric::from_str("").is_err());
    }

    #[test]
    fn all_weights_zero_is_treated_as_empty() {
        static METRIC_WITH_ZERO_STR: &str = "archived = 0.0";
        let metric = Metric::from_str(METRIC_WITH_ZERO_STR);
        assert!(metric.is_err());
    }

    #[test]
    fn unknown_field_is_error() {
        static WEIRD_METRIC: &str = r#"
            archived = 0.1
            definetelyNotAFieldThatSecureSumWouldExpectGivenThePresentStateOfTheEconomy = 0.2
        "#;
        assert!(Metric::from_str(WEIRD_METRIC).is_err());
    }

    #[test]
    fn metric_serialization_almost_roundtrip() {
        let expected_serialized = r#"
[[probe]]
name = "archived"
weight = -1.0

[[probe]]
name = "codeApproved"
weight = 1.0
"#;

        let metric = MetricNew {
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
        let serialized = toml::to_string(&metric).unwrap();

        assert_eq!(serialized.trim(), expected_serialized.trim());

        let result = toml::from_str::<MetricNew>(&serialized);
        assert!(result.is_ok());
    }
}
