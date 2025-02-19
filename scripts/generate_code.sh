#!/bin/bash

set -e

cd "$(git rev-parse --show-toplevel)"

if [ ! -d ./scripts/venv ]; then
    python -m venv ./scripts/venv
fi

if [ -d ./scripts/venv/bin ]; then
    source ./scripts/venv/bin/activate
else
    source ./scripts/venv/Scripts/activate
fi

pip install -r ./scripts/requirements.txt
python ./scripts/generate_code.py

cargo fmt
