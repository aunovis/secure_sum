#![warn(clippy::unwrap_used)]

mod args;
mod cumulated_probe;
mod ecosystem;
mod error;
mod filesystem;
mod github_token;
mod metric;
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
use metric::Metric;
use repodata::RepoData;
use scorecard::{dispatch_scorecard_runs, ensure_scorecard_binary};
use simple_logger::SimpleLogger;
use tabled::{Table, settings::Style};
use target::Target;

use crate::error::Error;

fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .init()
        .map_err(|e| Error::Other(e.to_string()))?;
    let args = Arguments::parse();
    let metric = Metric::new(args.metric.as_deref())?;
    let targets: Vec<_> = args
        .dependencies
        .into_iter()
        .map(|t| {
            let target = Target::parse(t)?;
            log::debug!("Parsed target: {target}");
            Ok(target)
        })
        .collect::<Result<_, Error>>()?;
    ensure_scorecard_binary()?;
    ensure_valid_github_token()?;
    let results = dispatch_scorecard_runs(&metric, targets, args.rerun)?;
    let mut results: Vec<_> = results.iter().map(|r| RepoData::new(r, &metric)).collect();
    results.sort();
    println!("{}", Table::new(&results).with(Style::rounded()));
    if args.details {
        for repo in results {
            repo.print_detailed_output();
        }
    }
    Ok(())
}
