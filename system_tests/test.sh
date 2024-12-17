#!/bin/bash

set -euo pipefail

# Go to git root folder
cd $(git rev-parse --show-toplevel)

# Print help
cargo run --release -- --help

# Load metrics and run probes
cargo run --release -- github.com/aunovis/secure_sum ./system_tests/example_metrics.toml
