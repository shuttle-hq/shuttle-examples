#!/usr/bin/env bash

set -euo pipefail
IFS=$'\n\t'

# Get all directories with Cargo.toml files
DIRS=$(find . -name Cargo.toml -exec dirname {} \;)

# Loop through each directory and run cargo fmt and cargo clippy, exit on failure
for dir in $DIRS; do
    if [ -f "$dir/.disable_ci" ]; then
        echo "Skipping $dir"
        continue
    fi

    echo "Checking $dir"
    if [ -f "$dir/.target" ]; then
        target=$(<$dir/.target)

        cargo fmt --all --manifest-path "$dir/Cargo.toml"
        # cargo clippy --no-deps --target $target --manifest-path "$dir/Cargo.toml" -- -D warnings
    else
        cargo fmt --all --manifest-path "$dir/Cargo.toml"
        # cargo clippy --no-deps --manifest-path "$dir/Cargo.toml" -- -D warnings
    fi
done