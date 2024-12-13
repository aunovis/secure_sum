#![warn(missing_docs)]

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Arguments {
    /// Path to the metrics file
    #[arg(short, long, value_name = "PATH")]
    pub metrics_file: PathBuf,
}
