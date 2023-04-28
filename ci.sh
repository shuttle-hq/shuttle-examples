#!/bin/bash
set -euo pipefail
IFS=$'\n\t'

# Get all directories with Cargo.toml files
DIRS=$(find . -name Cargo.toml -exec dirname {} \;)

# Loop through each directory and run cargo fmt and cargo clippy, exit on failure
for dir in $DIRS; do
    echo "Checking $dir"
    
    pushd $dir > /dev/null # Change to the directory
    
    cargo fmt --all -- --check
    cargo clippy --no-deps -- -D warnings
    
    popd > /dev/null # Go back to the previous directory
done