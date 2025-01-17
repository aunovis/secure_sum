use crate::{
    metric::Metric,
    probe::{ProbeFinding, ProbeOutcome},
};

static NORM: f32 = 10.;

#[derive(Debug, PartialEq)]
struct WeighedFinding {
    probe: String,
    weight: f32,
    outcome: ProbeOutcome,
}

fn weighed_findings(findings: &[ProbeFinding], metric: &Metric) -> Vec<WeighedFinding> {
    todo!()
}

fn calculate_total_score(findings: &[WeighedFinding]) -> f32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weigh_findings_ignores_probes_not_in_metric() {
        let metric = Metric {
            archived: Some(1.),
            ..Default::default()
        };
        let findings = vec![
            ProbeFinding {
                probe: "archived".to_owned(),
                outcome: ProbeOutcome::True,
            },
            ProbeFinding {
                probe: "fuzzed".to_owned(),
                outcome: ProbeOutcome::True,
            },
        ];

        let weighed = weighed_findings(&findings, &metric);

        let expected = vec![WeighedFinding {
            probe: "archived".to_owned(),
            outcome: ProbeOutcome::True,
            weight: 1.,
        }];
        assert_eq!(weighed, expected);
    }

    #[test]
    fn weighed_findings_are_sorted_by_weight_amplitude() {
        let metric = Metric {
            archived: Some(1.),
            fuzzed: Some(2.),
            codeApproved: Some(-3.),
            ..Default::default()
        };
        let findings = vec![
            ProbeFinding {
                probe: "archived".to_owned(),
                outcome: ProbeOutcome::True,
            },
            ProbeFinding {
                probe: "codeApproved".to_owned(),
                outcome: ProbeOutcome::True,
            },
            ProbeFinding {
                probe: "fuzzed".to_owned(),
                outcome: ProbeOutcome::True,
            },
        ];

        let weighed = weighed_findings(&findings, &metric);

        let expected = vec![
            WeighedFinding {
                probe: "codeApproved".to_owned(),
                outcome: ProbeOutcome::True,
                weight: -3.,
            },
            WeighedFinding {
                probe: "fuzzed".to_owned(),
                outcome: ProbeOutcome::True,
                weight: 2.,
            },
            WeighedFinding {
                probe: "archived".to_owned(),
                outcome: ProbeOutcome::True,
                weight: 1.,
            },
        ];
        assert_eq!(weighed, expected);
    }

    #[test]
    fn total_score_ignores_non_boolean_outcomes() {
        let findings = vec![
            WeighedFinding {
                probe: "archived".to_owned(),
                outcome: ProbeOutcome::True,
                weight: 1.,
            },
            WeighedFinding {
                probe: "codeApproved".to_owned(),
                outcome: ProbeOutcome::False,
                weight: 1.,
            },
            WeighedFinding {
                probe: "fuzzed".to_owned(),
                outcome: ProbeOutcome::NotSupported,
                weight: 1.,
            },
        ];

        assert_eq!(calculate_total_score(&findings), NORM / 2.);
    }

    #[test]
    fn total_score_is_normed() {
        let findings = vec![WeighedFinding {
            probe: "archived".to_owned(),
            outcome: ProbeOutcome::True,
            weight: 1.234,
        }];

        assert_eq!(calculate_total_score(&findings), NORM);
    }

    #[test]
    fn total_score_is_normed_between_min_and_max_value() {
        let findings = vec![
            WeighedFinding {
                probe: "archived".to_owned(),
                outcome: ProbeOutcome::True,
                weight: -1.,
            },
            WeighedFinding {
                probe: "codeApproved".to_owned(),
                outcome: ProbeOutcome::True,
                weight: 1.,
            },
            WeighedFinding {
                probe: "fuzzed".to_owned(),
                outcome: ProbeOutcome::True,
                weight: 1.,
            },
        ];
        // Lowest possible value before normalization is -1.
        // Highest is 2.
        // Actual i 1.
        // => After normalization, value is at 2/3 of NORM.

        assert_eq!(calculate_total_score(&findings), NORM * 2. / 3.);
    }

    #[test]
    fn total_score_can_handle_empty_findings() {
        assert_eq!(calculate_total_score(&vec![]), 0.);
    }
}
