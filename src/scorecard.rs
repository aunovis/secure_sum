use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use flate2::read::GzDecoder;
use tar::Archive;

use crate::{
    error::Error,
    filesystem::{data_dir, ARCH_STR, OS_STR},
    metric::Metric,
    probe::{load_stored_probe, needs_rerun, store_probe, ProbeResult},
    target::Target,
};

static CURRENT_VERSION: &str = "5.0.0";

fn scorecard_url() -> String {
    format!("https://github.com/ossf/scorecard/releases/download/v{CURRENT_VERSION}/scorecard_{CURRENT_VERSION}_{OS_STR}_{ARCH_STR}.tar.gz")
}

fn scorecard_path() -> Result<PathBuf, Error> {
    #[cfg(target_os = "windows")]
    let ending = ".exe";
    #[cfg(not(target_os = "windows"))]
    let ending = "";
    let executable_name = format!("scorecard-{OS_STR}-{ARCH_STR}{ending}");
    Ok(data_dir()?.join(executable_name))
}

pub(crate) fn ensure_scorecard_binary() -> Result<PathBuf, Error> {
    let path = scorecard_path()?;
    if fs::exists(&path)? {
        return Ok(path);
    }
    let dir = data_dir()?;
    fs::create_dir_all(&dir)?;
    let url = scorecard_url();
    log::info!("Downloading Scorecard binary from {url}");
    let mut response = reqwest::blocking::get(url)?;
    let gz_decoder = GzDecoder::new(&mut response);
    let mut archive = Archive::new(gz_decoder);
    archive.unpack(dir)?;
    log::info!("Stored binary under {}", path.display());
    Ok(path)
}

pub(crate) fn dispatch_scorecard_runs(
    metric: &Metric,
    target: Target,
    force_rerun: bool,
) -> Result<(), Error> {
    let scorecard = scorecard_path()?;
    log::debug!("Running scorecard binary {}", scorecard.display());
    match target {
        Target::Url(repo) => evaluate_repo(&repo, metric, &scorecard, force_rerun)?,
    };
    Ok(())
}

fn evaluate_repo(
    repo: &str,
    metric: &Metric,
    scorecard: &Path,
    force_rerun: bool,
) -> Result<ProbeResult, Error> {
    if !force_rerun {
        if let Some(stored_probe) = load_stored_probe(repo)? {
            if !needs_rerun(&stored_probe, metric) {
                return Ok(stored_probe);
            }
        }
    }
    run_scorecard_probe(repo, metric, scorecard)
}

fn run_scorecard_probe(
    repo: &str,
    metric: &Metric,
    scorecard: &Path,
) -> Result<ProbeResult, Error> {
    log::debug!("Checking {repo}");
    let args = scorecard_args(metric, repo)?;
    log::trace!("Args: {:#?}", args);
    let output = Command::new(scorecard).args(args).output()?;
    let stderr = String::from_utf8(output.stderr)?;
    if !stderr.is_empty() {
        log::error!("{stderr}");
        return Err(Error::Scorecard(stderr));
    }
    let stdout = String::from_utf8(output.stdout)?;
    let probe_result = serde_json::from_str(&stdout)?;
    store_probe(&stdout)?;
    Ok(probe_result)
}

fn scorecard_args(metric: &Metric, repo: &str) -> Result<Vec<String>, Error> {
    let mut args = vec![];
    args.push(format!("--repo={repo}"));
    let probes = metric
        .probes()
        .map(|(name, _)| name.to_string())
        .collect::<Vec<_>>();
    if probes.is_empty() {
        return Err(Error::Input(
            "At least one probe needs to be specified".to_owned(),
        ));
    }
    let probes = probes.join(",");
    args.push(format!("--probes={probes}"));
    args.push("--format=probe".to_string());
    Ok(args)
}

#[cfg(test)]
mod tests {
    use reqwest::blocking::Client;
    use serial_test::serial;

    use crate::probe::probe_file;

    use super::*;

    static EXAMPLE_REPO: &str = "https://github.com/aunovis/secure_sum";

    #[test]
    fn scorecard_url_exists() {
        let url = scorecard_url();
        let client = Client::new();
        let response = client.head(&url).send().unwrap();
        assert!(response.status().is_success(), "URL is: {url}")
    }

    #[test]
    fn data_dir_contains_aunovis_string() {
        let path = scorecard_path().unwrap().to_string_lossy().to_lowercase();
        assert!(path.contains("aunovis"), "Path is: {path}");
    }

    #[test]
    fn scorecard_path_contains_scorecard_string() {
        let path = scorecard_path().unwrap().to_string_lossy().to_lowercase();
        assert!(path.contains("scorecard"), "Path is: {path}");
    }

    #[test]
    #[serial]
    fn scorecard_binary_exists_after_ensure_scorecard_binary_call() {
        let path = ensure_scorecard_binary().expect("Ensuring scorecard binary failed");
        assert!(path.exists(), "Path is: {}", path.display());
        assert!(path.is_file(), "Path is: {}", path.display());
    }

    #[test]
    #[serial]
    fn scorecard_binary_can_be_executed_after_ensure_scorecard_binary_call() {
        let path = ensure_scorecard_binary().unwrap();
        let result = Command::new(path).arg("--version").output();
        assert!(result.is_ok(), "Error occurred: {}", result.unwrap_err())
    }

    #[test]
    fn scorecard_args_without_probes_is_err() {
        let metric = Metric::default();
        let args_result = scorecard_args(&metric, EXAMPLE_REPO);
        assert!(args_result.is_err())
    }

    #[test]
    fn scorecard_args_one_probe() {
        let metric = Metric {
            archived: Some(1.),
            ..Default::default()
        };
        let args = scorecard_args(&metric, EXAMPLE_REPO).unwrap();
        let expected = vec![
            format!("--repo={EXAMPLE_REPO}"),
            "--probes=archived".to_string(),
            "--format=probe".to_string(),
        ];
        assert_eq!(args, expected);
    }

    #[test]
    fn scorecard_args_several_probes() {
        let metric = Metric {
            archived: Some(1.),
            fuzzed: Some(1.3),
            ..Default::default()
        };
        let args = scorecard_args(&metric, EXAMPLE_REPO).unwrap();
        let expected = vec![
            format!("--repo={EXAMPLE_REPO}"),
            "--probes=archived,fuzzed".to_string(),
            "--format=probe".to_string(),
        ];
        assert_eq!(args, expected);
    }

    #[test]
    #[serial]
    fn running_scorecard_stores_output() {
        ensure_scorecard_binary().unwrap();
        dotenvy::dotenv().unwrap();
        let scorecard = scorecard_path().unwrap();
        let filepath = probe_file(EXAMPLE_REPO).unwrap();
        fs::remove_file(&filepath).ok();
        let metric = Metric {
            archived: Some(1.),
            ..Default::default()
        };
        assert!(!filepath.exists());
        let result = run_scorecard_probe(EXAMPLE_REPO, &metric, &scorecard);
        assert!(result.is_ok(), "{:#?}", result);
        assert!(filepath.exists(), "{} does not exist", filepath.display())
    }

    #[test]
    #[serial]
    fn running_scorecard_with_nonexistent_repo_produces_error() {
        ensure_scorecard_binary().unwrap();
        dotenvy::dotenv().unwrap();
        let scorecard = scorecard_path().unwrap();
        let metric = Metric {
            archived: Some(1.),
            ..Default::default()
        };
        let repo = "buubpvnuodypyocmqnhv";
        let result = run_scorecard_probe(repo, &metric, &scorecard);
        assert!(result.is_err(), "{:#?}", result.unwrap());
        let error_print = format!("{}", result.unwrap_err());
        assert!(error_print.contains(repo), "Error print is: {error_print}");
    }

    #[test]
    #[serial]
    fn running_scorecard_without_metrics_produces_error() {
        ensure_scorecard_binary().unwrap();
        dotenvy::dotenv().unwrap();
        let scorecard = scorecard_path().unwrap();
        let metric = Metric::default();
        let result = run_scorecard_probe(EXAMPLE_REPO, &metric, &scorecard);
        assert!(result.is_err(), "{:#?}", result.unwrap());
        let error_print = format!("{}", result.unwrap_err());
        assert!(
            error_print.contains("probe"),
            "Error print is: {error_print}"
        );
    }
}
