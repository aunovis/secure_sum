#![warn(clippy::unwrap_used)]

mod args;
mod error;
mod metrics;

use args::Arguments;
use clap::Parser;
use metrics::Metric;

fn main() -> Result<(), crate::error::Error> {
    let args = Arguments::parse();
    let metrics = Metric::from_file(&args.metrics_file)?;
    println!("Prased metrics: {:?}", metrics);
    Ok(())
}
