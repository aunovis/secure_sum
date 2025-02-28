#!/bin/bash

set -e

cd "$(git rev-parse --show-toplevel)"
cd scripts

source ./python_cmd.sh

$PYTHON -m pip install --upgrade pip

pip install -r ./requirements.txt

pip install --upgrade setuptools pip-review

pip-review --auto

pip freeze > requirements.txt
