use std::path::Path;

use crate::error::Error;

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
    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn metric_can_be_read_from_file() {
        let tempfile = NamedTempFile::new().unwrap();
        todo!()
    }

    #[test]
    fn all_probes_are_optional() {
        todo!()
    }

    #[test]
    fn completely_empty_metric_is_not_ok() {
        todo!();
    }
}
