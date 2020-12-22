#!/usr/bin/env bash
# fetch (clone) a list of repos.

set -e

die() { echo "🔥 Error: $*" 1>&2; exit 1; }

if [[ -z "$PANBUILD_REPOS_PATH" ]]; then
    die "Missing PANBUILD_REPOS_PATH environment variable!"
fi

pushd "$PANBUILD_REPOS_PATH"

while read line; do
  echo "$line"
done < "${1:-/dev/stdin}"

popd
