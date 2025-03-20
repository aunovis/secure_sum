#!/bin/bash

set -e

cd "$(git rev-parse --show-toplevel)"

cargo binstall cargo-dist

echo "Run"
echo "dist init"
echo "to (re)configure the release pipelines."

# I think linux uses '@' to separate package name from version,
# and windows uses '#'. This sed command can handle both.
PACKAGE_VERSION=$(cargo pkgid | sed 's/.*[@|#]//')
TAG="v${PACKAGE_VERSION}"
echo "Preparing release of: $TAG"
# Check if tag already exists
if git rev-parse $TAG >/dev/null 2>&1; then
   echo "Tag $TAG already exists. Exiting."
   exit 1
fi
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

git tag $TAG
git push --tags
