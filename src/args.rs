#![warn(missing_docs)]

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Arguments {
    /// Path to the metrics file that defines the probes to analyse
    #[arg(value_name = "FILEPATH")]
    pub(crate) metrics_file: PathBuf,
    
    /// Local Path to a dependencyfile or url to a single repository
    #[arg(value_name = "FILEPATH|URL")]
    pub(crate) dependencies: String,
}
