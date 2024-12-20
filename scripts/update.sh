#!/bin/bash

set -e

cd $(git rev-parse --show-toplevel)

python ./scripts/generate_code.py
cargo fmt

cargo update

cargo install cargo-outdated
cargo outdated --exit-code 1

./scripts/check_licenses.sh
