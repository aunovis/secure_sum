use std::{fs, path::PathBuf};

use chrono::NaiveDate;
use serde::Deserialize;

use crate::{
    error::Error,
    filesystem::{data_dir, OS_STR},
    metric::Metric,
};

#[derive(Deserialize, Debug)]
pub(crate) struct ProbeResult {
    date: NaiveDate,
    repo: Repo,
    findings: Vec<ProbeFinding>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Repo {
    name: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct ProbeFinding {
    probe: String,
    outcome: ProbeOutcome,
}

#[derive(Deserialize, Debug)]
pub(crate) enum ProbeOutcome {
    True,
    False,
}

pub(crate) fn probe_file(repo: &str) -> Result<PathBuf, Error> {
    let probe_dir = data_dir()?.join("probes");
    let sanitise_opts = sanitize_filename::Options {
        replacement: "", // TODO: Improve replacement.
        windows: cfg!(windows),
        truncate: false,
    };
    let filename = sanitize_filename::sanitize_with_options(repo, sanitise_opts);
    Ok(probe_dir.join(filename))
}

pub(crate) fn store_probe(raw_output: &str) -> Result<(), Error> {
    let parsed_probe: ProbeResult = serde_json::from_str(raw_output)?;
    let repo = &parsed_probe.repo.name;
    let path = probe_file(repo)?;
    if let Some(dir) = path.parent() {
        if !dir.exists() {
            fs::create_dir_all(dir)?;
        }
    }
    log::debug!("Storing probe output in {}", path.display());
    Ok(fs::write(path, raw_output)?)
}

fn load_stored_probe(repo: String) -> Result<Option<ProbeResult>, Error> {
    todo!()
}

fn needs_rerun(repo: &str, metric: &Metric, stored_probe: &ProbeResult) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::*;

    static EXAMPLE: &str = r#"
{
    "date": "2025-01-07",
    "repo": {
        "name": "github.com/aunovis/secure_sum",
        "commit": "9a004aeb9b6feb01e267f01d37865dec9df85227"
    },
    "scorecard": {
        "version": "v5.0.0",
        "commit": "ea7e27ed41b76ab879c862fa0ca4cc9c61764ee4"
    },
    "findings": [
        {
            "probe": "archived",
            "message": "Repository is not archived.",
            "outcome": "False"
        },
        {
            "remediation": {
                "text": "Setup one of tools we currently detect https://github.com/ossf/scorecard/blob/main/docs/checks/fuzzing/README.md.",
                "markdown": "Setup one of [tools we currently detect](https://github.com/ossf/scorecard/blob/main/docs/checks/fuzzing/README.md).",
                "effort": 3
            },
            "probe": "fuzzed",
            "message": "no fuzzer integrations found",
            "outcome": "False"
        }
    ]
}
    "#;

    #[test]
    fn example_can_be_deserialised() {
        let result: Result<ProbeResult, _> = serde_json::from_str(EXAMPLE);
        assert!(result.is_ok(), "{:#?}", result);
    }

    #[test]
    #[serial]
    fn store_probe_stores_probe() {
        let repo = "github.com/aunovis/secure_sum";
        let path = probe_file(repo).unwrap();
        fs::remove_file(&path).ok();

        assert!(!path.exists());
        store_probe(EXAMPLE).unwrap();
        assert!(path.exists(), "{} does not exist", path.display());
    }
}
