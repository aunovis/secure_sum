use std::cmp::Ordering;

use tabled::Tabled;

use crate::{
    cumulated_probe::{cumulated_outcomes, display_true_outcomes, CumulatedProbeOutcome},
    metric::Metric,
    probe::ProbeResult,
    score::{calculate_total_score, weighed_findings},
    url::Url,
};

#[derive(Debug, PartialEq, Tabled)]
pub(crate) struct RepoData {
    repo: Url,
    total_score: f32,
    #[tabled(display = "display_true_outcomes")]
    probe_outcomes: Vec<CumulatedProbeOutcome>,
}

impl Eq for RepoData {}

impl RepoData {
    pub(crate) fn new(result: &ProbeResult, metric: &Metric) -> Self {
        let findings = weighed_findings(&result.findings, metric);
        let total_score = calculate_total_score(&findings);
        let repo = result.repo.name.clone();
        let probe_outcomes = cumulated_outcomes(&findings);
        Self {
            total_score,
            repo,
            probe_outcomes,
        }
    }
}

impl Ord for RepoData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match other.total_score.partial_cmp(&self.total_score) {
            Some(Ordering::Equal) | None => {}
            Some(ord) => return ord,
        }
        self.repo.cmp(&other.repo)
    }
}

impl PartialOrd for RepoData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repodata_is_sorted_in_descending_order() {
        let mut data = vec![
            RepoData {
                total_score: 1.,
                repo: "1".into(),
                probe_outcomes: vec![],
            },
            RepoData {
                total_score: 2.,
                repo: "2".into(),
                probe_outcomes: vec![],
            },
        ];
        data.sort();
        assert_eq!(data[0].repo.0, "2");
        assert_eq!(data[1].repo.0, "1");
    }
}
