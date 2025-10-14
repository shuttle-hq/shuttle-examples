#!/usr/bin/env bash
# Run with:
#   bash _scripts/sync-all-template-repos.sh
# Dependencies:
# - rust-script
# - + all dependencies of _scripts/sync-template-repo.sh

set -euo pipefail

for line in $(rust-script _scripts/check-templates.rs -- --list-template-repos); do
    name="$(echo "$line" | cut -d'|' -f1)"
    path="$(echo "$line" | cut -d'|' -f2)"
    echo "Synching $name $path"
    bash _scripts/sync-template-repo.sh "$name" "$path"
done
