#!/bin/bash

set -e

# Change to git root directory
cd "$(git rev-parse --show-toplevel)"

# Node.js
curl https://raw.githubusercontent.com/TheComamba/UnKenny/refs/heads/main/package.json > ./system_tests/example_package.json

# NuGet project.assets.json
curl https://gist.githubusercontent.com/nkolev92/becd4854beaadb9ab53c22f5ccb689f9/raw/80fff62a49ec17935baea516e801908c9ba7f067/sample.project.assets.json > ./system_tests/project.assets.json

# NuGet packages.config
curl https://raw.githubusercontent.com/dotnet/aspnetcore/refs/heads/main/eng/common/sdl/packages.config > ./system_tests/packages.config
