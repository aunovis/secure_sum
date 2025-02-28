#!/bin/bash

set -e

cd "$(git rev-parse --show-toplevel)"
cd scripts

source ./python_cmd.sh

pip install -r ./requirements.txt
$PYTHON ./generate_code.py

cargo fmt
