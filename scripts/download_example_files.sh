#!/bin/bash

set -e

# Change to git root directory
cd "$(git rev-parse --show-toplevel)"

# Node.js
curl https://raw.githubusercontent.com/TheComamba/UnKenny/refs/heads/main/package.json > ./system_tests/node_js_package.json

# NuGet project.csproj
curl https://raw.githubusercontent.com/JamesNK/Newtonsoft.Json/refs/heads/master/Src/Newtonsoft.Json/Newtonsoft.Json.csproj > ./system_tests/nuget_project_1.csproj
curl https://raw.githubusercontent.com/protobuf-net/protobuf-net/refs/heads/main/Build.csproj > ./system_tests/nuget_project_2.csproj

# NuGet packages.config
curl https://raw.githubusercontent.com/DotNetAnalyzers/StyleCopAnalyzers/refs/heads/master/.nuget/packages.config > ./system_tests/nuget_packages.config

# Rust (Another crate's Cargo.toml)
curl https://raw.githubusercontent.com/TheComamba/LoreCore/refs/heads/main/Cargo.toml >  ./system_tests/rust_cargo.toml
