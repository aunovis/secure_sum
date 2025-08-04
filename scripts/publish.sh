#!/bin/bash

set -e

# Check if there are any uncommitted changes
if ! git diff-index --quiet HEAD --; then
   echo "There are uncommitted changes. Exiting."
   exit 1
fi
# Check if this is the main branch
if [ "$(git rev-parse --abbrev-ref HEAD)" != "main" ]; then
   echo "Not on main branch. Exiting."
   exit 1
fi

if [ ! -f ~/.cargo/credentials.toml ]
then
    echo "You are not logged in to cargo."
    echo "Please visit "
    echo "https://crates.io/me"
    echo "to generate a token with the publish-update capability, and then run "
    echo "cargo login <your-api-token>"
    echo "to log in."
    exit 1
fi

cargo install cargo-release

if [[ "$1" == "--execute" ]]; then
    cargo release --execute
    echo "The GitHub release is handled by the dist pipeline."
    echo "Do NOT execute it manually, or it will interfere with that."
else
    cargo release
    echo "The script was run without --execute argument. If you want to execute the release, run the script with --execute argument."
fi
