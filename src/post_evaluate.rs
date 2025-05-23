use crate::{Arguments, Error, Metric, RepoData};

static DEFAULT_ERROR_THRESHOLD: f32 = 3.0;

struct Thresholds {
    error: f32,
    warn: f32,
}

pub(crate) fn post_evaluate_repos(
    results: &[RepoData],
    metric: &Metric,
    args: &Arguments,
) -> Result<(), Error> {
    let thresholds = get_thresholds(metric, args);
    let error_threshold = thresholds.error;
    let warn_threshold = thresholds.warn;
    let mut contains_error = false;
    for result in results {
        let score = result.score();
        let repo = result.repo();
        if score < error_threshold {
            log::error!(
                "Repo {repo} has a score of {score}, which is below the error threshold of {error_threshold}."
            );
            contains_error = true;
        } else if score < warn_threshold {
            log::warn!(
                "Repo {repo} has a score of {score}, which is dangerously close to the error threshold of {error_threshold}.;"
            )
        }
    }
    if contains_error {
        Err(Error::ScoreTooLow)
    } else {
        Ok(())
    }
}

fn get_thresholds(metric: &Metric, args: &Arguments) -> Thresholds {
    let mut error_threshold = args.error_threshold;
    let mut warn_threshold = args.warn_threshold;
    if error_threshold.is_none() {
        error_threshold = metric.error_threshold;
    }
    if warn_threshold.is_none() {
        warn_threshold = metric.warn_threshold;
    }
    let error = error_threshold.unwrap_or(DEFAULT_ERROR_THRESHOLD);
    let warn = warn_threshold.unwrap_or_else(|| error + 1.);

    if warn < error {
        log::warn!("Warning threshold is below error threshold, it will never become relevant.");
    }
    if warn < 0. {
        log::error!("Warning threshold is below 0.");
    }
    if error < 0. {
        log::error!("Error threshold is below 0.");
    }
    if warn > 10. {
        log::error!("Warning threshold is above 10.");
    }
    if error > 10. {
        log::error!("Error threshold is above 10.");
    }
    Thresholds { error, warn }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn float_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < 0.01
    }

    #[test]
    fn get_thresholds_returns_3_n_4_without_specifications() {
        let metric = Metric {
            error_threshold: None,
            warn_threshold: None,
            probes: vec![],
        };
        let args = Arguments::default();
        let thresholds = get_thresholds(&metric, &args);
        assert!(float_eq(thresholds.error, 3.0));
        assert!(float_eq(thresholds.warn, 4.0));
    }

    #[test]
    fn get_thresholds_returns_values_specified_in_metric() {
        let metric = Metric {
            error_threshold: Some(1.2),
            warn_threshold: Some(3.4),
            probes: vec![],
        };
        let args = Arguments::default();
        let thresholds = get_thresholds(&metric, &args);
        assert!(float_eq(thresholds.error, 1.2));
        assert!(float_eq(thresholds.warn, 3.4));
    }

    #[test]
    fn get_thresholds_overwrites_metric_values_by_args() {
        let metric = Metric {
            error_threshold: Some(1.2),
            warn_threshold: Some(3.4),
            probes: vec![],
        };
        let args = Arguments {
            error_threshold: Some(5.6),
            warn_threshold: Some(7.8),
            ..Default::default()
        };
        let thresholds = get_thresholds(&metric, &args);
        assert!(float_eq(thresholds.error, 5.6));
        assert!(float_eq(thresholds.warn, 7.8));
    }

    #[test]
    fn get_thresholds_sets_unspecified_warn_to_error_plus_1() {
        let metric = Metric {
            error_threshold: Some(1.2),
            warn_threshold: None,
            probes: vec![],
        };
        let mut args = Arguments {
            error_threshold: None,
            warn_threshold: None,
            ..Default::default()
        };
        let thresholds = get_thresholds(&metric, &args);
        assert!(float_eq(thresholds.warn, 2.2));
        args.error_threshold = Some(5.6);
        let thresholds = get_thresholds(&metric, &args);
        assert!(float_eq(thresholds.warn, 6.6));
    }
}
