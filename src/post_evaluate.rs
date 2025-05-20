use crate::{Arguments, Error, Metric, RepoData};

struct Thresholds {
    error: f32,
    warn: f32,
}

pub(crate) fn post_evaluate_repos(
    results: &[RepoData],
    metric: &Metric,
    args: &Arguments,
) -> Result<(), Error> {
    todo!()
}

fn get_thresholds(metric: &Metric, args: &Arguments) -> Thresholds {
    todo!("print warn if error > warn, error if any <0 or >10")
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
        assert!(float_eq(thresholds.warn, 6.6));
    }
}
