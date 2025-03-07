#!/bin/bash

set -e

cd "$(git rev-parse --show-toplevel)"

cargo build --release
# Checks all first level dependencies plus the repo itself
time cargo run --release -- ./reasonable_default_metrics.toml ./Cargo.toml https://github.com/aunovis/secure_sum --details
