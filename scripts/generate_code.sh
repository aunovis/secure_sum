#!/bin/bash

set -e

cd "$(git rev-parse --show-toplevel)"

source ./scripts/python_cmd.sh

pip install -r ./scripts/requirements.txt
$PYTHON ./scripts/generate_code.py

cargo fmt
