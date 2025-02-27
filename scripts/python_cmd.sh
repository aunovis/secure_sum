#!/bin/bash

set -e

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    PYTHON="python3"
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" || "$OSTYPE" == "win32" ]]; then
    PYTHON="python"
else
    echo "Unsupported OS"
    exit 1
fi

git_root="$(git rev-parse --show-toplevel)"

if [ ! -d $git_root/scripts/venv ]; then
    $PYTHON -m venv $git_root/scripts/venv
fi

if [ -d $git_root/scripts/venv/bin ]; then
    source $git_root/scripts/venv/bin/activate
else
    source $git_root/scripts/venv/Scripts/activate
fi
