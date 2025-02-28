use std::cmp::Ordering;

use crate::{
    metric::Metric,
    probe::{ProbeFinding, ProbeOutcome},
};

static NORM: f32 = 10.;
static ZERO_ACCURACY: f32 = 1e-10;

#[derive(Debug, PartialEq)]
pub(crate) struct WeighedFinding {
    probe: String,
    weight: f32,
    outcome: ProbeOutcome,
}

impl Eq for WeighedFinding {}

impl Ord for WeighedFinding {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_weight = self.weight.abs();
        let other_weight = other.weight.abs();
        // The order is such that higher values come first.
        match other_weight.partial_cmp(&self_weight) {
            Some(Ordering::Equal) | None => {}
            Some(ord) => return ord,
        }
        self.probe.cmp(&other.probe)
    }
}

impl PartialOrd for WeighedFinding {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub(crate) fn weighed_findings(findings: &[ProbeFinding], metric: &Metric) -> Vec<WeighedFinding> {
    let mut weighed = vec![];
    for probe in metric.probes {
        let finding = findings.iter().find(|f| f.probe == &probe.name);
        let outcome = match finding {
            Some(finding) => finding.outcome,
            None => {
                log::error!("Findings contain no outcome for probe \"{}\"", probe.name);
                continue;
            }
        };
        weighed.push(WeighedFinding {
            probe: probe.to_owned(),
            weight,
            outcome,
        });
    }
    weighed.sort();
    weighed
}

pub(crate) fn calculate_total_score(findings: &[WeighedFinding]) -> f32 {
    let (lowest, highest) = lowest_and_highest_possible_value(findings);
    let translation_offset = -lowest;
    let scale = highest - lowest;
    if scale.abs() < ZERO_ACCURACY {
        log::warn!("A finding has a difference of {scale} between lowest and highest possible value. Most probably too many probes yielded no boolean result.");
        return 0.;
    }
    let factor = NORM / scale;
    let mut weighed_sum: f32 = findings
        .iter()
        .filter_map(|f| {
            if f.outcome == ProbeOutcome::True {
                Some(f.weight)
            } else {
                None
            }
        })
        .sum();
    weighed_sum += translation_offset;
    weighed_sum *= factor;
    weighed_sum
}

pub(crate) fn boolean_outcomes(findings: &[WeighedFinding]) -> Vec<&WeighedFinding> {
    findings.iter().filter(|f| f.outcome.is_boolean()).collect()
}

fn lowest_and_highest_possible_value(findings: &[WeighedFinding]) -> (f32, f32) {
    let lowest = boolean_outcomes(findings)
        .iter()
        .filter_map(|f| if f.weight < 0. { Some(f.weight) } else { None })
        .sum();
    let highest = boolean_outcomes(findings)
        .iter()
        .filter_map(|f| if f.weight > 0. { Some(f.weight) } else { None })
        .sum();
    (lowest, highest)
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

        let (lowest, highest) = lowest_and_highest_possible_value(&findings);
        assert_eq!(lowest, 0.,);
        assert_eq!(highest, 2.);
        assert_eq!(calculate_total_score(&findings), NORM / 2.);
    }

    #[test]
    fn total_score_is_normed() {
        let findings = vec![WeighedFinding {
            probe: "archived".to_owned(),
            outcome: ProbeOutcome::True,
            weight: 1.234,
        }];

        let (lowest, highest) = lowest_and_highest_possible_value(&findings);
        assert_eq!(lowest, 0.,);
        assert_eq!(highest, 1.234);
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

        let (lowest, highest) = lowest_and_highest_possible_value(&findings);
        assert_eq!(lowest, -1.,);
        assert_eq!(highest, 2.);
        // Actual result is 1, which is 2/3 along the scale.
        assert_eq!(calculate_total_score(&findings), NORM * 2. / 3.);
    }

    #[test]
    fn total_score_can_handle_empty_findings() {
        let (lowest, highest) = lowest_and_highest_possible_value(&vec![]);
        assert_eq!(lowest, 0.,);
        assert_eq!(highest, 0.);
        assert_eq!(calculate_total_score(&vec![]), 0.);
    }
}
