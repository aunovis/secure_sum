#!/bin/bash

set -e

cd "$(git rev-parse --show-toplevel)"

cargo build --release
time cargo run --release -- ./reasonable_default_metrics.toml ./Cargo.toml
