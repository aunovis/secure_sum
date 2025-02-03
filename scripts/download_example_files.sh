#!/bin/bash

set -e

# Change to git root directory
cd "$(git rev-parse --show-toplevel)"

curl https://raw.githubusercontent.com/TheComamba/UnKenny/refs/heads/main/package.json > ./system_tests/example_package.json
