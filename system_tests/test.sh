#!/bin/bash

set -eou pipefail

# Go to git root folder
cd "$(git rev-parse --show-toplevel)"

# Print help
cargo run --release -- --help

# Load metrics and run probes on a single repository
cargo run --release -- ./system_tests/example_metrics.toml https://github.com/aunovis/secure_sum

# The second time around, the stored results are used
cargo run --release -- ./system_tests/example_metrics.toml https://github.com/aunovis/secure_sum

# Unless the --rerun flag is used
cargo run --release -- ./system_tests/example_metrics.toml https://github.com/aunovis/secure_sum --rerun
