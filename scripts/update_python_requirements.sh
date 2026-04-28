#!/bin/bash

set -e

cd "$(git rev-parse --show-toplevel)"
cd scripts

source ./python_cmd.sh

yesterday=$(date -Is -d "yesterday")

$PYTHON -m pip install --upgrade --uploaded-prior-to=$yesterday pip

pip install --upgrade --uploaded-prior-to=$yesterday --requirement ./requirements.txt

pip freeze > requirements.txt
