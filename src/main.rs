#![warn(clippy::unwrap_used)]

mod args;
mod ecosystem;
mod error;
mod filesystem;
mod metric;
mod metric_impl;
mod probe;
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
    let metrics = Metric::from_file(&args.metrics_file)?;
    log::debug!("Parsed metrics:\n{metrics}");
    let target = Target::parse(args.dependencies)?;
    log::debug!("Parsed target: {target}");
    ensure_scorecard_binary()?;
    dotenvy::dotenv()?;
    let results = dispatch_scorecard_runs(&metrics, target, args.rerun)?;
    let results: Vec<_> = results.iter().map(|r| RepoData::new(r, &metrics)).collect();
    log::info!("\n{}", Table::new(results));
    Ok(())
}
