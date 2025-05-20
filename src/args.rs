#![warn(missing_docs)]

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[derive(Default)]
pub(crate) struct Arguments {
    /// Path to the metric file that defines the probes to analyse
    #[arg(long, short, value_name = "FILEPATH")]
    pub(crate) metric: Option<PathBuf>,

    /// Local Path to a dependencyfile or url to a single repository
    #[arg(value_name = "FILEPATH(S)|URL(S)")]
    pub(crate) dependencies: Vec<String>,

    /// Print a detailed output which probes yielded which results for which repo
    #[arg(long, short)]
    pub(crate) details: bool,

    /// Rerun all scorecard checks, even if recent results are stored locally
    #[arg(long, short)]
    pub(crate) rerun: bool,

    /// Supress all output except for results and errors
    #[arg(long, short)]
    pub(crate) quiet: bool,

    /// Print a lot of detailed output
    #[arg(long, short)]
    pub(crate) verbose: bool,

    /// Overwrite the minimal score a repo must reach before an error is displayed.
    #[arg(long, short)]
    pub(crate) error_threshold: Option<f32>,

    /// Overwrite the minimal score a repo must reach before a warning is displayed.
    #[arg(long, short)]
    pub(crate) warn_threshold: Option<f32>,
}
