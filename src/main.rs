#![warn(clippy::unwrap_used)]

mod args;
mod error;
mod metrics;
mod scorecard;

use args::Arguments;
use clap::Parser;
use metrics::Metric;
use scorecard::ensure_scorecard_binary;
use simple_logger::SimpleLogger;

use crate::error::Error;

fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .init()
        .map_err(|e| Error::Other(e.to_string()))?;
    let args = Arguments::parse();
    let metrics = Metric::from_file(&args.metrics_file)?;
    log::debug!("Parsed metrics: {:?}", metrics);
    ensure_scorecard_binary()?;
    Ok(())
}
