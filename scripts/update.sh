#!/bin/bash

set -e

cd "$(git rev-parse --show-toplevel)"

echo Updating Python Packages
chmod +x ./scripts/update_python_requirements.sh
./scripts/update_python_requirements.sh

echo Generating Code
chmod +x ./scripts/generate_code.sh
./scripts/generate_code.sh

echo Installing cargo tools
cargo install cargo-cooldown --locked
cargo install cargo-outdated --locked

echo Updating Dependencies
cargo cooldown update

echo Checking for outdated dependencies
cargo outdated
read -p "Do any requirements need to be updated in the Cargo.toml? (Press any key to proceed)"

echo Checking for compile errors
cargo check --all

echo Checking for license policy violations
chmod +x ./scripts/check_licenses.sh
./scripts/check_licenses.sh

echo Checking for uncommitted changes
git diff --ignore-cr-at-eol -G. --exit-code
