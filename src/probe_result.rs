use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct ProbeResult {
    date: DateTime<Utc>,
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

#[cfg(test)]
mod tests {
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
}
