#!/bin/bash

set -euo pipefail

# Go to git root folder
cd $(git rev-parse --show-toplevel)

# Print help
cargo run --release -- --help

# Load metrics
cargo run --release -- --metrics-file ./system_tests/example_metrics.toml
