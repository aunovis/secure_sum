#!/bin/bash

set -eou pipefail

# Go to git root folder
cd "$(git rev-parse --show-toplevel)"
cd system_tests

# Print help
cargo run --release -- --help

# Run probes with default metric on a single repository
cargo run --release -- https://github.com/aunovis/secure_sum
# The second time around, the stored results are used
cargo run --release -- https://github.com/aunovis/secure_sum
# Unless the --rerun flag is used
cargo run --release -- https://github.com/aunovis/secure_sum --rerun
# The --details flag offers more detailed output on how each probe contributed
cargo run --release -- https://github.com/aunovis/secure_sum --details
# Load a custom metric
cargo run --release -- --metric example_metric.toml https://github.com/aunovis/secure_sum

# Run on a dependencyfile of the Rust ecosystem
cargo run --release -- --metric example_metric.toml rust_cargo.toml
# Run on a dependencyfile of the Node.js ecosystem, with verbose output and radically low error thresholds
cargo run --release -- --metric example_metric.toml node_js_package.json --verbose --error-threshold=0.1
# Run on a project.csproj file of the NuGet ecosystem, with less output
cargo run --release -- --metric example_metric.toml nuget_project_1.csproj --quiet -e 1.5
# Run on a (somewhat old-fashioned) packages.config file of the NuGet ecosystem
cargo run --release -- --metric example_metric.toml nuget_packages.config

# You can specify more than one dependencyfile
cargo run --release -- --metric example_metric.toml ../Cargo.toml rust_cargo.toml
# They may even be from different ecosystem, if for some reason that makes sense in your project
cargo run --release -- --metric example_metric.toml rust_cargo.toml node_js_package.json
# Or mix and match a dependencyfile with URLs
cargo run --release -- --metric example_metric.toml rust_cargo.toml https://github.com/aunovis/secure_sum
# You can use some bash scripting to include all .csproj files
cargo run --release -- --metric example_metric.toml $(find . -iname "*.csproj")
