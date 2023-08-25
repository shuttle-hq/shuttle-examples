#!/usr/bin/env bash

set -euo pipefail
IFS=$'\n\t'

# Get all directories with Cargo.toml files
DIRS=$(find . -name Cargo.toml -exec dirname {} \;)

# Loop through each directory and run cargo fmt and cargo clippy, exit on failure
for dir in $DIRS; do
    echo "Checking $dir"
    
    cargo fmt --all --manifest-path "$dir/Cargo.toml" -- --check
    cargo clippy --no-deps --manifest-path "$dir/Cargo.toml" -- -D warnings
done