use std::{fs::read_to_string, path::Path};

use serde::Deserialize;

use crate::error::Error;

#[derive(PartialEq, Deserialize)]
#[allow(non_snake_case)]
pub(crate) struct Metric {
    #[serde(default)]
    archived: Option<f32>,
    #[serde(default)]
    blocksDeleteOnBranches: Option<f32>,
}

impl Metric {
    fn from_file(filepath: &Path) -> Result<Self, Error> {
        let content = read_to_string(filepath)?;
        Self::from_str(&content)
    }

    fn from_str(str: &str) -> Result<Self, Error> {
        toml::from_str(str)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{remove_file, write};

    use tempfile::NamedTempFile;

    use super::*;

    static EXAMPLE_METRIC: Metric = Metric {
        archived: Some(0.1),
        blocksDeleteOnBranches: Some(0.2),
    };

    static EXAMPLE_METRIC_STR: &str = r#"
        archived = 0.1
        blocksDeleteOnBranches = 0.2
    "#;

    #[test]
    fn metric_can_be_read_from_file() {
        let tempfile = NamedTempFile::new().unwrap();
        let filepath = tempfile.path();
        write(path, EXAMPLE_METRIC_STR).unwrap();
        let metric = Metric::from_file(path).expect("Failed to read from file!");
        assert!(metric, EXAMPLE_METRIC);

        remove_file(path);
    }

    #[test]
    fn all_probes_are_optional() {
        static ONLY_ONE_PROBE: &str = "archived = 0.1";
        let metric = Metric::from_str(ONLY_ONE_PROBE).expect("Failed to parse metric!");
        assert!(metric.archived = Some(0.1));
        assert!(metric.blocksDeleteOnBranches = None);

        static ONLY_ONE_OTHER_PROBE: &str = "blocksDeleteOnBranches = 0.2";
        let metric = Metric::from_str(ONLY_ONE_OTHER_PROBE).expect("Failed to parse metric!");
        assert!(metric.archived = None);
        assert!(metric.blocksDeleteOnBranches = Some(0.2));
    }

    #[test]
    fn completely_empty_metric_is_not_ok() {
        assert!(Metric::from_str("").is_err());
    }

    #[test]
    fn unknown_field_is_error() {
        static WEIRD_METRIC: &str = r#"
            archived = 0.1
            definetelyNotAFieldThatSecureSumWouldExpectGivenThePresentStateOfTheEconomy = 0.2
        "#;
        assert!(Metric::from_str(WEIRD_METRIC).is_err());
    }
}
