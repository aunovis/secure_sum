#!/bin/bash

set -e

cd $(git rev-parse --show-toplevel)
cd scripts

if [ ! -d venv ]; then
    python -m venv venv
fi

if [ -d ./venv/bin ]; then
    source ./venv/bin/activate
else
    source ./venv/Scripts/activate
fi

pip install -r requirements.txt
python ./generate_code.py

cargo fmt
