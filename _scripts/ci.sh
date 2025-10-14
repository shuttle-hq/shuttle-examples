#!/usr/bin/env bash

set -euo pipefail
IFS=$'\n\t'

# Get all directories with Cargo.toml files, alphabetically between $1 and $2
FROM="./$1"
TO="./$2"
DIRS=$(find . -name Cargo.toml -exec dirname {} \; | sort | awk -v from="$FROM" -v to="$TO" '($0 >= from) && ($0 < to) {print $0}')

# Loop through each directory and run cargo fmt and cargo clippy, exit on failure
for dir in $DIRS; do
    # skip workspace roots
    grep "\[workspace\]" "$dir/Cargo.toml" > /dev/null && continue

    echo "Checking $dir"
    cargo fmt --all --manifest-path "$dir/Cargo.toml" -- --check
    cargo clippy --no-deps --manifest-path "$dir/Cargo.toml" -- -D warnings
done
