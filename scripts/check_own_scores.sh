#!/bin/bash

set -e

cd "$(git rev-parse --show-toplevel)"

cargo build --release
# Checks all first level dependencies plus the repo itself
cargo run --release -- ./Cargo.toml https://github.com/aunovis/secure_sum --details
