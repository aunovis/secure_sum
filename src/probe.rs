use std::{
    fs::{self, read_to_string},
    path::PathBuf,
};

use chrono::{Duration, NaiveDate, Utc};
use serde::Deserialize;

use crate::{error::Error, filesystem::data_dir, metric::Metric};

static PROBE_VALIDITY_PERIOD: Duration = Duration::weeks(1);

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct ProbeResult {
    date: NaiveDate,
    repo: Repo,
    findings: Vec<ProbeFinding>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct Repo {
    name: String,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct ProbeFinding {
    probe: String,
    outcome: ProbeOutcome,
}

#[derive(Clone, Copy, Deserialize, Debug, PartialEq, Eq)]
pub(crate) enum ProbeOutcome {
    True,
    False,
}

pub(crate) fn probe_file(repo: &str) -> Result<PathBuf, Error> {
    let probe_dir = data_dir()?.join("probes");
    static HTTP: &str = "http://";
    static HTTPS: &str = "https://";
    let no_protocol = if repo.starts_with(HTTP) {
        &repo[HTTP.len()..]
    } else if repo.starts_with(HTTPS) {
        &repo[HTTPS.len()..]
    } else {
        repo
    };
    // Dots are valid in filenames, but without this replacement almost every probe has basename "github".
    let no_dots = no_protocol.replace(".", "_");
    let sanitise_opts = sanitize_filename::Options {
        replacement: "_", // Replace invalid characters with underscores
        windows: cfg!(windows),
        truncate: false,
    };
    let filename = sanitize_filename::sanitize_with_options(no_dots, sanitise_opts);
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

fn load_stored_probe(repo: &str) -> Result<Option<ProbeResult>, Error> {
    let path = probe_file(repo)?;
    if !path.exists() {
        return Ok(None);
    }
    let contents = read_to_string(path)?;
    let probe = serde_json::from_str(&contents)?;
    Ok(Some(probe))
}

fn needs_rerun(stored_probe: &ProbeResult, metric: &Metric) -> bool {
    let today = Utc::now().date_naive();
    let time_since_last_check = today.signed_duration_since(stored_probe.date);
    if time_since_last_check >= PROBE_VALIDITY_PERIOD {
        return true;
    }
    let probe_finding_names: Vec<_> = stored_probe
        .findings
        .iter()
        .map(|f| f.probe.as_str())
        .collect();
    let mut probes_to_run = metric.probes();
    probes_to_run.any(|(probe, _)| !probe_finding_names.contains(&probe))
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};
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
    fn probe_filename_removes_protocol() {
        let no_protocol = probe_file("test.com/path").unwrap();
        let http_protocol = probe_file("http://test.com/path").unwrap();
        let https_protocol = probe_file("https://test.com/path").unwrap();
        assert_eq!(no_protocol, http_protocol);
        assert_eq!(no_protocol, https_protocol);
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

    #[test]
    #[serial]
    fn load_probe_loads_probe_if_it_exists() {
        let repo = "github.com/aunovis/secure_sum";
        let path = probe_file(repo).unwrap();

        fs::remove_file(&path).ok();
        assert!(!path.exists());

        let probe = load_stored_probe(repo).unwrap();
        assert!(probe.is_none());

        store_probe(EXAMPLE).unwrap();
        assert!(path.exists());

        let probe = load_stored_probe(repo).unwrap().unwrap();
        let expected = serde_json::from_str(EXAMPLE).unwrap();
        assert_eq!(probe, expected);
    }

    #[test]
    fn probe_needs_rerun_if_result_is_older_than_validity() {
        let today = Utc::now().date_naive();
        let yesterday = (Utc::now() - Duration::days(1)).date_naive();
        let yesterweek = (Utc::now() - Duration::weeks(1)).date_naive();
        let mut probe = ProbeResult {
            date: today,
            repo: Repo {
                name: "Some Repo".to_owned(),
            },
            findings: vec![ProbeFinding {
                probe: "archived".to_owned(),
                outcome: ProbeOutcome::True,
            }],
        };
        let metric = Metric::from_str("archived = 1").unwrap();

        assert!(!needs_rerun(&probe, &metric));

        probe.date = yesterday;
        assert!(!needs_rerun(&probe, &metric));

        probe.date = yesterweek;
        assert!(needs_rerun(&probe, &metric));
    }

    #[test]
    fn probe_needs_rerun_if_metric_contains_probes_without_finding() {
        let metric = Metric::from_str("archived = 1\ncodeApproved = 1").unwrap();
        let same_findings = vec![
            ProbeFinding {
                probe: "archived".to_owned(),
                outcome: ProbeOutcome::True,
            },
            ProbeFinding {
                probe: "codeApproved".to_owned(),
                outcome: ProbeOutcome::True,
            },
        ];
        let other_finding = ProbeFinding {
            probe: "fuzzed".to_owned(),
            outcome: ProbeOutcome::True,
        };
        let less_findings = same_findings[1..].to_vec();
        let mut more_findings = same_findings.clone();
        more_findings.push(other_finding.clone());
        let mut other_findings = same_findings.clone();
        other_findings[0] = other_finding;

        let mut probe = ProbeResult {
            date: Utc::now().date_naive(),
            repo: Repo {
                name: "Some Repo".to_owned(),
            },
            findings: vec![],
        };

        probe.findings = same_findings;
        assert!(!needs_rerun(&probe, &metric));
        probe.findings = less_findings;
        assert!(needs_rerun(&probe, &metric));
        probe.findings = more_findings;
        assert!(!needs_rerun(&probe, &metric));
        probe.findings = other_findings;
        assert!(needs_rerun(&probe, &metric));
    }
}
