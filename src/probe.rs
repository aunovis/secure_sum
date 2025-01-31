use std::{
    fs::{self, read_to_string},
    path::PathBuf,
};

use chrono::{Duration, NaiveDate, Utc};
use serde::Deserialize;

use crate::{error::Error, filesystem::data_dir, metric::Metric, target::SingleTarget, url::Url};

static PROBE_VALIDITY_PERIOD: Duration = Duration::weeks(1);

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct ProbeResult {
    date: NaiveDate,
    pub(crate) repo: Repo,
    pub(crate) findings: Vec<ProbeFinding>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct Repo {
    pub(crate) name: Url,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct ProbeFinding {
    pub(crate) probe: String,
    pub(crate) outcome: ProbeOutcome,
}

/// Corresponds to constants defined in https://github.com/ossf/scorecard/blob/main/finding/finding.go
#[derive(Clone, Copy, Deserialize, Debug, PartialEq, Eq)]
pub(crate) enum ProbeOutcome {
    False,
    NotAvailable,
    Error,
    True,
    NotSupported,
    NotApplicable,
}

impl ProbeOutcome {
    pub(crate) fn is_boolean(&self) -> bool {
        self == &ProbeOutcome::True || self == &ProbeOutcome::False
    }
}

pub(crate) fn probe_file(target: &SingleTarget) -> Result<PathBuf, Error> {
    let probe_dir = data_dir()?.join("probes");
    let package = match target {
        SingleTarget::Package(package, ecosystem) => format!("{}_{package}", ecosystem.as_str()),
        SingleTarget::Url(url) => url.str_without_protocol().to_owned(),
    };
    // Dots are valid in filenames, but without this replacement almost every probe has basename "github".
    let no_dots = package.replace(".", "_");
    let sanitise_opts = sanitize_filename::Options {
        replacement: "_", // Replace invalid characters with underscores
        windows: cfg!(windows),
        truncate: false,
    };
    let mut filename = sanitize_filename::sanitize_with_options(no_dots, sanitise_opts);
    filename.push_str(".json");
    Ok(probe_dir.join(filename))
}

pub(crate) fn store_probe(target: &SingleTarget, raw_output: &str) -> Result<(), Error> {
    let path = probe_file(target)?;
    if let Some(dir) = path.parent() {
        if !dir.exists() {
            fs::create_dir_all(dir)?;
        }
    }
    log::debug!("Storing probe output in {}", path.display());
    Ok(fs::write(path, raw_output)?)
}

pub(crate) fn load_stored_probe(target: &SingleTarget) -> Result<Option<ProbeResult>, Error> {
    let path = probe_file(target)?;
    if !path.exists() {
        return Ok(None);
    }
    let contents = read_to_string(path)?;
    let probe = serde_json::from_str(&contents)?;
    Ok(Some(probe))
}

pub(crate) fn needs_rerun(stored_probe: &ProbeResult, metric: &Metric) -> bool {
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

    use crate::ecosystem::Ecosystem;

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

    fn url_target(string: &str) -> SingleTarget {
        SingleTarget::Url(Url(string.to_owned()))
    }

    #[test]
    fn example_can_be_deserialised() {
        let result: Result<ProbeResult, _> = serde_json::from_str(EXAMPLE);
        assert!(result.is_ok(), "{:#?}", result);
    }

    #[test]
    fn probe_filename_removes_protocol_from_url() {
        let no_protocol = probe_file(&url_target("test.com/path")).unwrap();
        let http_protocol = probe_file(&url_target("http://test.com/path")).unwrap();
        let https_protocol = probe_file(&url_target("https://test.com/path")).unwrap();
        assert_eq!(no_protocol, http_protocol);
        assert_eq!(no_protocol, https_protocol);
    }

    #[test]
    fn probe_filename_adds_ecosystem() {
        let target = SingleTarget::Package("serde".to_owned(), Ecosystem::Rust);
        let path = probe_file(&target).unwrap();
        let file = path.file_name().unwrap().to_str().unwrap().to_owned();
        assert_eq!(file, "rust_serde.json");
    }

    #[test]
    #[serial]
    fn store_probe_stores_probe() {
        let repo = url_target("github.com/aunovis/secure_sum");
        let path = probe_file(&repo).unwrap();
        fs::remove_file(&path).ok();

        assert!(!path.exists());
        store_probe(&repo, EXAMPLE).unwrap();
        assert!(path.exists(), "{} does not exist", path.display());
    }

    #[test]
    #[serial]
    fn load_probe_loads_probe_if_it_exists() {
        let repo = url_target("github.com/aunovis/secure_sum");
        let path = probe_file(&repo).unwrap();

        fs::remove_file(&path).ok();
        assert!(!path.exists());

        let probe = load_stored_probe(&repo).unwrap();
        assert!(probe.is_none());

        store_probe(&repo, EXAMPLE).unwrap();
        assert!(path.exists());

        let probe = load_stored_probe(&repo).unwrap().unwrap();
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
                name: "Some Repo".into(),
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
                name: "Some Repo".into(),
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
