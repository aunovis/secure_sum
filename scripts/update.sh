#!/bin/bash

set -e

cd $(git rev-parse --show-toplevel)

echo Updating Python Packages
chmod +x ./scripts/update_python_requirements.sh
./scripts/update_python_requirements.sh

echo Generating Code
chmod +x ./scripts/generate_code.sh
./scripts/generate_code.sh

echo Updating Dependencies
cargo update

echo Checking for outdated dependencies
cargo install cargo-outdated
cargo outdated --exit-code 1

echo Checking for license policy violations
chmod +x ./scripts/check_licenses.sh
./scripts/check_licenses.sh
