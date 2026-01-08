use std::{
    fs,
    io::{BufReader, Read},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread::{self, JoinHandle},
    time,
};

use flate2::read::GzDecoder;
use rayon::prelude::*;
use tar::Archive;
use wait_timeout::ChildExt;

use crate::{
    error::Error,
    filesystem::{ARCH_STR, OS_STR, data_dir},
    http::http_get,
    metric::Metric,
    probe::{ProbeResult, load_stored_probe, needs_rerun, store_probe, store_probe_json},
    target::{SingleTarget, Target, collect_single_targets},
};

static CURRENT_VERSION: &str = "5.4.0";
/// The default timeout seems very high. That's because scorecard can be really slow.
static DEFAULT_TIMEOUT: time::Duration = time::Duration::from_secs(180);

fn scorecard_url() -> String {
    format!(
        "https://github.com/ossf/scorecard/releases/download/v{CURRENT_VERSION}/scorecard_{CURRENT_VERSION}_{OS_STR}_{ARCH_STR}.tar.gz"
    )
}

fn scorecard_path() -> Result<PathBuf, Error> {
    #[cfg(target_os = "windows")]
    let ending = ".exe";
    #[cfg(not(target_os = "windows"))]
    let ending = "";
    let executable_name = format!("scorecard{ending}");
    Ok(data_dir()?.join(executable_name))
}

fn scorecard_path_with_arch() -> Result<PathBuf, Error> {
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
    log::info!("Downloading Scorecard binary from {url}.");
    let mut response = http_get(&url, None)?;
    let mut response_reader = response.body_mut().as_reader();
    let gz_decoder = GzDecoder::new(&mut response_reader);
    let mut archive = Archive::new(gz_decoder);
    archive.unpack(dir)?;
    if !fs::exists(&path)? {
        let path_with_arch = scorecard_path_with_arch()?;
        if fs::exists(&path_with_arch)? {
            fs::rename(path_with_arch, &path)?;
        } else {
            let mut message = String::new();
            message.push_str("The downloaded archive was successfully unpacked in \"");
            message.push_str(&path.to_string_lossy());
            message.push_str("\", but the binary could not be located automatically. ");
            message.push_str("Please go to the folder, and rename the binary to \"");
            message.push_str(&path.to_string_lossy());
            message.push_str("\".");
            log::error!("{message}");
            return Err(Error::Other(message));
        }
    }
    log::info!("Stored binary under \"{}\".", path.display());
    Ok(path)
}

pub(crate) fn dispatch_scorecard_runs(
    metric: &Metric,
    targets: Vec<Target>,
    force_rerun: bool,
    timeout: Option<humantime::Duration>,
) -> Result<Vec<ProbeResult>, Error> {
    let scorecard = scorecard_path()?;
    log::debug!("Running scorecard binary {}", scorecard.display());
    let results = collect_single_targets(targets)
        .par_iter()
        .map(|target| evaluate_repo(target, metric, &scorecard, force_rerun, timeout))
        .collect::<Result<_, _>>()?;
    Ok(results)
}

fn evaluate_repo(
    target: &SingleTarget,
    metric: &Metric,
    scorecard: &Path,
    force_rerun: bool,
    timeout: Option<humantime::Duration>,
) -> Result<ProbeResult, Error> {
    if !force_rerun {
        if let Some(stored_probe) = load_stored_probe(target)? {
            if !needs_rerun(&stored_probe, metric) {
                return Ok(stored_probe);
            }
        }
    }
    let timeout = timeout
        .map(|duration| time::Duration::from_secs(duration.as_secs()))
        .unwrap_or(DEFAULT_TIMEOUT);
    run_scorecard_probe(target, metric, scorecard, timeout)
}

fn run_scorecard_probe(
    target: &SingleTarget,
    metric: &Metric,
    scorecard: &Path,
    timeout: time::Duration,
) -> Result<ProbeResult, Error> {
    log::info!("Evaluating {target}.");
    let args = scorecard_args(metric, target)?;
    log::debug!("Args: {:#?}", args);

    let output = wait_for_scorecard_evaluation(scorecard, timeout, args)?;

    let stderr = output.1;
    if !stderr.is_empty() {
        log::error!("Scorecard reported an error: {stderr}");
        let probe_result = ProbeResult::from_scorecard_error(target, stderr);
        store_probe(target, &probe_result)?;
        return Ok(probe_result);
    }
    let stdout = output.0;
    let probe_result = serde_json::from_str(&stdout)?;
    store_probe_json(target, &stdout)?;
    log::info!("Finished evaluation of {target}.");
    Ok(probe_result)
}

fn wait_for_scorecard_evaluation(
    scorecard: &Path,
    timeout: time::Duration,
    args: Vec<String>,
) -> Result<(String, String), Error> {
    let mut child = Command::new(scorecard)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let stdout = child.stdout.take().ok_or(Error::Other(
        "Could not access stdout of child process.".to_string(),
    ))?;
    let stderr = child.stderr.take().ok_or(Error::Other(
        "Could not access stderr of child process.".to_string(),
    ))?;

    let stdout_handle = thread::spawn(move || {
        let mut out = String::new();
        BufReader::new(stdout).read_to_string(&mut out)?;
        Ok(out)
    });

    let stderr_handle = thread::spawn(move || {
        let mut err = String::new();
        BufReader::new(stderr).read_to_string(&mut err)?;
        Ok(err)
    });

    let output = match child.wait_timeout(timeout)? {
        Some(code) => {
            log::debug!("Scorecard process finished in time with {code}.");
            let stdout = join_output(stdout_handle)?;
            let stderr = join_output(stderr_handle)?;
            (stdout, stderr)
        }
        None => {
            let timeout = humantime::Duration::from(timeout);
            log::error!("Scorecard process timed out after {timeout}.");
            child.kill()?;
            child.wait()?;
            return Err(Error::Timeout);
        }
    };
    Ok(output)
}

fn join_output(handle: JoinHandle<Result<String, Error>>) -> Result<String, Error> {
    match handle.join() {
        Ok(outcome) => outcome,
        Err(_) => Err(Error::Other("Could not join output handle.".to_string())),
    }
}

fn scorecard_args(metric: &Metric, target: &SingleTarget) -> Result<Vec<String>, Error> {
    let mut args = vec![];
    args.push(target.to_scorecard_arg()?);
    let probes = metric
        .probes
        .iter()
        .map(|input| input.name.to_string())
        .collect::<Vec<_>>();
    if probes.is_empty() {
        return Err(Error::Input(
            "At least one probe needs to be specified.".to_owned(),
        ));
    }
    let probes = probes.join(",");
    args.push(format!("--probes={probes}"));
    args.push("--format=probe".to_string());
    Ok(args)
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use crate::{
        probe::{ProbeInput, probe_file},
        probe_name::ProbeName,
        url::Url,
    };

    use super::*;

    fn example_url() -> Url {
        Url("https://github.com/aunovis/secure_sum".to_string())
    }

    fn example_target() -> SingleTarget {
        SingleTarget::Url(example_url())
    }

    fn example_target_arg() -> String {
        format!("--repo=https://github.com/aunovis/secure_sum")
    }

    #[test]
    fn scorecard_url_exists() {
        let url = scorecard_url();
        let response = http_get(&url, None);
        assert!(response.is_ok());
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
        let metric = Metric {
            warn_threshold: None,
            error_threshold: None,
            probes: vec![],
        };

        let args_result = scorecard_args(&metric, &example_target());

        assert!(args_result.is_err())
    }

    #[test]
    fn scorecard_args_one_probe() {
        let metric = Metric {
            warn_threshold: None,
            error_threshold: None,
            probes: vec![ProbeInput {
                name: ProbeName::archived,
                weight: 1.,
                max_times: None,
            }],
        };

        let args = scorecard_args(&metric, &example_target()).unwrap();

        let expected = vec![
            example_target_arg(),
            "--probes=archived".to_string(),
            "--format=probe".to_string(),
        ];
        assert_eq!(args, expected);
    }

    #[test]
    fn scorecard_args_several_probes() {
        let metric = Metric {
            warn_threshold: None,
            error_threshold: None,
            probes: vec![
                ProbeInput {
                    name: ProbeName::archived,
                    weight: 1.,
                    max_times: None,
                },
                ProbeInput {
                    name: ProbeName::fuzzed,
                    weight: 1.3,
                    max_times: None,
                },
            ],
        };

        let args = scorecard_args(&metric, &example_target()).unwrap();

        let expected = vec![
            example_target_arg(),
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
        let filepath = probe_file(&example_target()).unwrap();
        fs::remove_file(&filepath).ok();
        let metric = Metric {
            warn_threshold: None,
            error_threshold: None,
            probes: vec![ProbeInput {
                name: ProbeName::archived,
                weight: 1.,
                max_times: None,
            }],
        };
        assert!(!filepath.exists());

        let result = run_scorecard_probe(&example_target(), &metric, &scorecard, DEFAULT_TIMEOUT);

        assert!(result.is_ok(), "{:#?}", result);
        assert!(filepath.exists(), "{} does not exist", filepath.display())
    }

    #[test]
    #[serial]
    fn scorecard_probe_on_unknown_repo_stores_error_in_result() {
        ensure_scorecard_binary().unwrap();
        dotenvy::dotenv().unwrap();
        let wrong_target = SingleTarget::Url(Url("https://ffzotuwjbbuxirheajde.com".to_string()));
        let filepath = probe_file(&wrong_target).unwrap();
        let scorecard = scorecard_path().unwrap();
        let metric = Metric {
            warn_threshold: None,
            error_threshold: None,
            probes: vec![ProbeInput {
                name: ProbeName::archived,
                weight: 1.,
                max_times: None,
            }],
        };

        let result = run_scorecard_probe(&wrong_target, &metric, &scorecard, DEFAULT_TIMEOUT);

        assert!(result.is_ok(), "{:#?}", result);
        assert!(filepath.exists(), "{} does not exist", filepath.display())
    }

    #[test]
    #[serial]
    fn running_scorecard_without_metric_produces_error() {
        ensure_scorecard_binary().unwrap();
        dotenvy::dotenv().unwrap();
        let scorecard = scorecard_path().unwrap();
        let metric = Metric {
            warn_threshold: None,
            error_threshold: None,
            probes: vec![],
        };

        let result = run_scorecard_probe(&example_target(), &metric, &scorecard, DEFAULT_TIMEOUT);

        assert!(result.is_err(), "{:#?}", result.unwrap());
        let error_print = format!("{}", result.unwrap_err());
        assert!(
            error_print.contains("probe"),
            "Error print is: {error_print}"
        );
    }

    #[test]
    #[serial]
    fn evaluation_is_aborted_after_timeout() {
        ensure_scorecard_binary().unwrap();
        dotenvy::dotenv().unwrap();
        let scorecard = scorecard_path().unwrap();
        let metric = Metric {
            warn_threshold: None,
            error_threshold: None,
            probes: vec![ProbeInput {
                name: ProbeName::dependencyUpdateToolConfigured,
                weight: 1.,
                max_times: None,
            }],
        };
        let way_too_short = time::Duration::from_nanos(10);

        let result = run_scorecard_probe(&example_target(), &metric, &scorecard, way_too_short);

        assert!(result.is_err(), "{:#?}", result.unwrap());
        let err = result.unwrap_err();
        assert!(matches!(err, Error::Timeout));
    }
}
