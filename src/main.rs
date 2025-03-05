#![warn(clippy::unwrap_used)]

mod args;
mod cumulated_probe;
mod ecosystem;
mod error;
mod filesystem;
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
use metric::Metric;
use repodata::RepoData;
use scorecard::{dispatch_scorecard_runs, ensure_scorecard_binary};
use simple_logger::SimpleLogger;
use tabled::Table;
use target::Target;

use crate::error::Error;

fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .init()
        .map_err(|e| Error::Other(e.to_string()))?;
    let args = Arguments::parse();
    let metric = Metric::from_file(&args.metric_file)?;
    log::debug!("Parsed metric:\n{metric}");
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
    dotenvy::dotenv().ok();
    let results = dispatch_scorecard_runs(&metric, targets, args.rerun)?;
    let mut results: Vec<_> = results.iter().map(|r| RepoData::new(r, &metric)).collect();
    results.sort();
    log::info!("\n{}", Table::new(results));
    Ok(())
}
