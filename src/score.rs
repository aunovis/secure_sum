use crate::{
    metric::Metric,
    probe::{ProbeFinding, ProbeOutcome},
};

static NORM: f32 = 10.;

struct WeighedFinding {
    probe: String,
    weight: f32,
    outcome: ProbeOutcome,
}

fn weigh_findings(result: &Vec<ProbeFinding>, metric: Metric) -> Vec<WeighedFinding> {
    todo!()
}

fn calculate_total_score(findings: &Vec<WeighedFinding>) -> f32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weig_findings_ignores_probes_not_in_metric() {
        todo!()
    }

    #[test]
    fn weighed_findingss_are_sorted_by_weight() {
        todo!()
    }

    #[test]
    fn total_score_is_normed() {
        todo!()
    }

    #[test]
    fn total_score_can_handle_division_by_zero() {
        todo!()
    }
}
