#!/usr/bin/env bash
# Run with:
#   bash _scripts/sync-template-repo.sh [template name] [path]
#     where [template name] is the hashmap key in templates.toml
#     and [path] is the path attribute of the template
# Dependencies:
# - shuttle CLI (no auth)
# - gh CLI (authenticated with sufficient permissions for the org)
# - git (will create commits with your default name+email)

set -euo pipefail

OWNER="shuttle-hq"
NAME="$1"
TEMPLATE_PATH="$2"

export GH_PAGER=""

set +e
repo=$(gh repo create "$OWNER/$NAME" --public)
if echo "$repo" | grep "Name already exists" -q; then
    # GraphQL: Name already exists on this account (createRepository)
    echo "already exists"
fi
set -e

# add admin permission for core and devrel teams
gh api -X PUT -H "Accept: application/vnd.github+json" "/orgs/$OWNER/teams/core/repos/$OWNER/$NAME" -f permission=admin
gh api -X PUT -H "Accept: application/vnd.github+json" "/orgs/$OWNER/teams/devrel/repos/$OWNER/$NAME" -f permission=admin

# set repo to be a template
gh api -X PATCH -H "Accept: application/vnd.github+json" "/repos/$OWNER/$NAME" -f is_template=true

tmp=$(mktemp -d)

shuttle init --from "./$TEMPLATE_PATH" --name "$NAME" $tmp
pushd $tmp

# get a fresh Cargo.lock
cargo update

# add template info to end of README
echo "

## Shuttle template: $NAME

This template ([repo](https://github.com/$OWNER/$NAME)) is a synced replica from [shuttle-examples](https://github.com/shuttle-hq/shuttle-examples/tree/main/$TEMPLATE_PATH).

[Deploy on Shuttle with just a few clicks!](https://console.shuttle.dev/templates/$NAME)

" >> README.md

git add .
git commit -am "Sync $NAME with shuttle-hq/shuttle-examples repo"

git remote add origin git@github.com:$OWNER/$NAME.git
git branch -M main
git push -u origin main --force

popd
