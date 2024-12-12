use std::path::Path;

use crate::error::Error;

#[derive(PartialEq)]
#[allow(non_snake_case)]
pub(crate) struct Metrics {
    archived: Option<f32>,
    blocksDeleteOnBranches: Option<f32>,
}

impl Metrics {
    fn from_file(filepath: &Path) -> Result<Self, Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::fs::write;

    use tempfile::NamedTempFile;

    use super::*;

    static EXAMPLE_METRIC: Metrics = Metrics {
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
        let metrics = Metrics::from_file(path).expect("Failed to read from file!");
        assert!(metrics, EXAMPLE_METRIC);
    }

    #[test]
    fn archived_probe_is_optional() {
        todo!()
    }

    #[test]
    fn completely_empty_metric_is_not_ok() {
        todo!();
    }
}
