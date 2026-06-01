#!/bin/bash

set -e

cd "$(git rev-parse --show-toplevel)"
cd scripts

source ./python_cmd.sh

$PYTHON -m pip install -r ./requirements.txt

yesterday=$(date -u -d "1 day ago" +"%Y-%m-%dT%H:%M:%S")
$PYTHON -m pip install --upgrade --uploaded-prior-to=$yesterday pip

outdated=$(pip list --outdated --format=json)
if [[ "$outdated" != "[]" ]]; then
    echo "$outdated" \
        | $PYTHON -c "import json,sys; print(' '.join(p['name'] for p in json.load(sys.stdin)))" \
        | xargs pip install --upgrade --uploaded-prior-to=$yesterday
else
    echo "Everything is up to date."
fi

pip freeze > requirements.txt
