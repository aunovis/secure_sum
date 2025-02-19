#!/bin/bash

set -e

cd "$(git rev-parse --show-toplevel)"
cd scripts

if [ ! -d venv ]; then
    python -m venv venv
fi

if [ -d ./venv/bin ]; then
    source ./venv/bin/activate
else
    source ./venv/Scripts/activate
fi

python.exe -m pip install --upgrade pip
pip install --upgrade setuptools pip-review

pip-review --auto

pip freeze > requirements.txt
