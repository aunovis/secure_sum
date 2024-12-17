#!/bin/bash

set -e

cd $(git rev-parse --show-toplevel)

cargo update

python ./scripts/generate_code.py

cargo fmt
