#![warn(clippy::unwrap_used)]

mod args;
mod cumulated_probe;
mod ecosystem;
mod error;
mod filesystem;
mod github_token;
mod logging;
mod metric;
mod post_evaluate;
mod probe;
mod probe_name;
mod repodata;
mod score;
mod scorecard;
mod target;
mod url;

use args::Arguments;
use clap::Parser;
use github_token::ensure_valid_github_token;
use logging::init_logging;
use metric::Metric;
use post_evaluate::post_evaluate_repos;
use repodata::RepoData;
use scorecard::{dispatch_scorecard_runs, ensure_scorecard_binary};
use tabled::{Table, settings::Style};
use target::Target;

use crate::error::Error;

fn main() -> Result<(), Error> {
    let args = Arguments::parse();
    if args.quiet && args.verbose {
        return Err(Error::Other(
            "Commandline arguments --quiet and --verbose can not be combined.".to_string(),
        ));
    }
    init_logging(&args)?;
    let metric = Metric::new(args.metric.as_deref())?;
    let targets: Vec<_> = args
        .dependencies
        .clone()
        .into_iter()
        .map(|t| {
            let target = Target::parse(t)?;
            log::debug!("Parsed target: {target}");
            Ok(target)
        })
        .collect::<Result<_, Error>>()?;
    ensure_scorecard_binary()?;
    ensure_valid_github_token()?;
    let results = dispatch_scorecard_runs(&metric, targets, args.rerun, args.timeout)?;
    let mut results: Vec<_> = results.iter().map(|r| RepoData::new(r, &metric)).collect();
    results.sort();
    println!("{}", Table::new(&results).with(Style::rounded()));
    if args.details {
        for repo in &results {
            repo.print_detailed_output();
        }
    }
    post_evaluate_repos(&results, &metric, &args)
}
