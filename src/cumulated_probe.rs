use tabled::Tabled;

use crate::{probe_name::ProbeName, score::WeighedFinding};

#[derive(Debug, PartialEq, Tabled)]
pub(crate) struct CumulatedProbeOutcome {
    probe: ProbeName,
    weight: f32,
    #[tabled(display = "display_option_usize")]
    max_times: Option<usize>,
    #[tabled(display = "display_option_usize")]
    true_outcomes: Option<usize>,
}

fn display_option_usize(option: &Option<usize>) -> String {
    match option {
        Some(value) => value.to_string(),
        None => "N/A".to_string(),
    }
}

fn cumulated_outcomes(findings: &[WeighedFinding]) -> Vec<CumulatedProbeOutcome> {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::probe::ProbeOutcome;

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
        todo!()
    }
}
