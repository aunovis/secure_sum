#![warn(clippy::unwrap_used)]

mod args;
mod error;
mod filesystem;
mod metric;
mod metric_impl;
mod probe;
mod scorecard;
mod target;

use args::Arguments;
use clap::Parser;
use metric::Metric;
use scorecard::{dispatch_scorecard_runs, ensure_scorecard_binary};
use simple_logger::SimpleLogger;
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
    dispatch_scorecard_runs(&metrics, target)?;
    Ok(())
}
