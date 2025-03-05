use std::collections::HashMap;

use tabled::Tabled;

use crate::{probe::ProbeOutcome, probe_name::ProbeName, score::WeighedFinding};

#[derive(Debug, PartialEq, Tabled)]
pub(crate) struct CumulatedProbeOutcome {
    #[tabled(rename = "Probe")]
    probe: ProbeName,
    #[tabled(rename = "Weight")]
    weight: f32,
    #[tabled(display = "display_option_usize", rename = "True Outcomes")]
    true_outcomes: Option<usize>,
}

fn display_option_usize(option: &Option<usize>) -> String {
    match option {
        Some(value) => value.to_string(),
        None => "N/A".to_string(),
    }
}

pub(crate) fn cumulated_outcomes(findings: &[WeighedFinding]) -> Vec<CumulatedProbeOutcome> {
    let mut outcomes_map: HashMap<ProbeName, (f32, Option<usize>)> = HashMap::new();

    for finding in findings {
        let entry = outcomes_map
            .entry(finding.probe)
            .or_insert((finding.weight, None));
        entry.1 = if finding.outcome == ProbeOutcome::False {
            Some(0)
        } else if finding.outcome == ProbeOutcome::True {
            match entry.1 {
                None => Some(1),
                Some(prev) => Some(prev + 1),
            }
        } else {
            None
        }
    }

    let mut outcomes: Vec<CumulatedProbeOutcome> = outcomes_map
        .into_iter()
        .map(|(probe, (weight, true_outcomes))| CumulatedProbeOutcome {
            probe,
            weight,
            true_outcomes,
        })
        .collect();

    outcomes.sort_by(|a, b| a.probe.cmp(&b.probe));
    outcomes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cumulated_outcomes_cumulates_outcomes() {
        let findings = [
            WeighedFinding {
                probe: ProbeName::archived,
                weight: -1.,
                outcome: ProbeOutcome::False,
            },
            WeighedFinding {
                probe: ProbeName::hasOSVVulnerabilities,
                weight: -1.,
                outcome: ProbeOutcome::True,
            },
            WeighedFinding {
                probe: ProbeName::hasOSVVulnerabilities,
                weight: -1.,
                outcome: ProbeOutcome::True,
            },
        ];

        let cumulated = cumulated_outcomes(&findings);

        assert_eq!(cumulated.len(), 2);
    }

    #[test]
    fn cumulated_outcomes_counts_true_outcomes() {
        let findings = [
            WeighedFinding {
                probe: ProbeName::hasOSVVulnerabilities,
                weight: -1.,
                outcome: ProbeOutcome::True,
            },
            WeighedFinding {
                probe: ProbeName::hasOSVVulnerabilities,
                weight: -1.,
                outcome: ProbeOutcome::True,
            },
        ];

        let cumulated = cumulated_outcomes(&findings);

        assert_eq!(cumulated[0].true_outcomes.unwrap(), 2);
    }

    #[test]
    fn cumulated_outcomes_counts_0_for_false_outcome() {
        let findings = [WeighedFinding {
            probe: ProbeName::hasOSVVulnerabilities,
            weight: -1.,
            outcome: ProbeOutcome::False,
        }];

        let cumulated = cumulated_outcomes(&findings);

        assert_eq!(cumulated[0].true_outcomes.unwrap(), 0);
    }

    #[test]
    fn cumulated_outcomes_counts_none_for_non_boolean_outcome() {
        let findings = [WeighedFinding {
            probe: ProbeName::hasOSVVulnerabilities,
            weight: -1.,
            outcome: ProbeOutcome::NotApplicable,
        }];

        let cumulated = cumulated_outcomes(&findings);

        assert!(cumulated[0].true_outcomes.is_none());
    }

    #[test]
    fn cumulated_outcomes_are_sorted_alphabetically() {
        let findings = [
            WeighedFinding {
                probe: ProbeName::blocksDeleteOnBranches,
                weight: 1.,
                outcome: ProbeOutcome::False,
            },
            WeighedFinding {
                probe: ProbeName::archived,
                weight: -1.,
                outcome: ProbeOutcome::True,
            },
            WeighedFinding {
                probe: ProbeName::hasOSVVulnerabilities,
                weight: -1.,
                outcome: ProbeOutcome::True,
            },
        ];

        let cumulated = cumulated_outcomes(&findings);

        assert_eq!(cumulated[0].probe, ProbeName::archived);
        assert_eq!(cumulated[1].probe, ProbeName::blocksDeleteOnBranches);
        assert_eq!(cumulated[2].probe, ProbeName::hasOSVVulnerabilities);
    }
}
