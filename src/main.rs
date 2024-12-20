#![warn(clippy::unwrap_used)]

mod args;
mod error;
mod input;
mod metric;
mod metric_impl;
mod scorecard;

use args::Arguments;
use clap::Parser;
use input::Input;
use metric::Metric;
use scorecard::ensure_scorecard_binary;
use simple_logger::SimpleLogger;

use crate::error::Error;

fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .init()
        .map_err(|e| Error::Other(e.to_string()))?;
    let args = Arguments::parse();
    let metrics = Metric::from_file(&args.metrics_file)?;
    log::debug!("Parsed metrics:\n{metrics}");
    let input = Input::parse(args.dependencies)?;
    log::debug!("Parsed input: {input}");
    ensure_scorecard_binary()?;
    Ok(())
}
