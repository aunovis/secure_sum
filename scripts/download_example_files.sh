#!/bin/bash

set -e

# Change to git root directory
cd "$(git rev-parse --show-toplevel)"

# Node.js
curl https://raw.githubusercontent.com/TheComamba/UnKenny/refs/heads/main/package.json > ./system_tests/node_js_package.json

# NuGet packages.config
curl https://raw.githubusercontent.com/DotNetAnalyzers/StyleCopAnalyzers/refs/heads/master/.nuget/packages.config > ./system_tests/nuget_packages.config
