#!/usr/bin/env bash
# fetch (clone) a list of repos.

set -e

die() { echo "🔥 Error: $*" 1>&2; exit 1; }

if [[ -z "$PANBUILD_REPOS_PATH" ]]; then
    die "Missing PANBUILD_REPOS_PATH environment variable!"
fi

pushd "$PANBUILD_REPOS_PATH"

while read repo; do
  if git clone "$repo" 2> /dev/null; then
      echo "Cloned repo $repo"
  fi
done < "${1:-/dev/stdin}"

popd
