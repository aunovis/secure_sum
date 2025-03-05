#![warn(missing_docs)]

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Arguments {
    /// Path to the metric file that defines the probes to analyse
    #[arg(value_name = "FILEPATH")]
    pub(crate) metric_file: PathBuf,

    /// Local Path to a dependencyfile or url to a single repository
    #[arg(value_name = "FILEPATH(S)|URL(S)")]
    pub(crate) dependencies: Vec<String>,

    /// Rerun all scorecard checks, even if recent results are stored locally
    #[arg(long, short)]
    pub(crate) rerun: bool,

    /// Print a detailed output which probes yielded which results for which repo
    #[arg(long, short)]
    pub(crate) details: bool,
}
