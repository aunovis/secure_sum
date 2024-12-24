use std::{fs, path::PathBuf};

use etcetera::{choose_app_strategy, AppStrategy, AppStrategyArgs};
use flate2::read::GzDecoder;
use tar::Archive;

use crate::{error::Error, metric::Metric, target::Target};

static CURRENT_VERSION: &str = "5.0.0";

#[cfg(target_os = "macos")]
static OS_STR: &str = "darwin";
#[cfg(target_os = "linux")]
static OS_STR: &str = "linux";
#[cfg(target_os = "windows")]
static OS_STR: &str = "windows";

/// target_arch config is not recognised on all OSs.
/// We therefore only check for "arm or not arm".-
#[cfg(target_arch = "arm")]
static ARCH_STR: &str = "arm64";
#[cfg(not(target_arch = "arm"))]
static ARCH_STR: &str = "amd64";

fn scorecard_url() -> String {
    format!("https://github.com/ossf/scorecard/releases/download/v{CURRENT_VERSION}/scorecard_{CURRENT_VERSION}_{OS_STR}_{ARCH_STR}.tar.gz")
}

fn data_dir() -> Result<PathBuf, Error> {
    let app_strategy_args = AppStrategyArgs {
        top_level_domain: "de".to_string(),
        author: "aunovis".to_string(),
        app_name: "aunovis_secure_sum".to_string(),
    };
    let data_dir = choose_app_strategy(app_strategy_args)
        .map_err(|e| Error::Other(e.to_string()))?
        .data_dir();
    Ok(data_dir)
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

pub(crate) fn run_scorecard(metric: &Metric, target: Target) {
    match target {
        Target::Url(url) => {
            let args = scorecard_args(metric, &url);
        }
    }
}

fn scorecard_args(metric: &Metric, repo: &str) -> Vec<String> {
    let mut args = vec![];
    args.push(format!("--repo={repo}"));
    let probes = metric
        .probes()
        .map(|(name, _)| name.to_string())
        .collect::<Vec<_>>()
        .join(",");
    args.push(format!("--probes={probes}"));
    args.push("--format=probe".to_string());
    args
}

#[cfg(test)]
mod tests {
    use reqwest::blocking::Client;

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
    fn scorecard_binary_exists_after_ensure_scorecard_binary_call() {
        let path = ensure_scorecard_binary().expect("Ensuring scorecard binary failed");
        assert!(path.exists(), "Path is: {}", path.display());
        assert!(path.is_file(), "Path is: {}", path.display());
    }

    #[test]
    fn scorecard_args_one_probe() {
        let metric = Metric {
            archived: Some(1.),
            ..Default::default()
        };
        let args = scorecard_args(&metric, EXAMPLE_REPO);
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
        let args = scorecard_args(&metric, EXAMPLE_REPO);
        let expected = vec![
            format!("--repo={EXAMPLE_REPO}"),
            "--probes=archived,fuzzed".to_string(),
            "--format=probe".to_string(),
        ];
        assert_eq!(args, expected);
    }
}
