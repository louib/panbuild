#!/usr/bin/env bash

FIXTURES_DIRS="./fixtures/"

echo "🔍 Starting functional test suite for 2flatpak."
fixtures_files=$(find "$FIXTURES_DIR")

IFS=$'\n'; for file in $fixtures_files; do
    if [[ -d "$file" ]]; then
        continue
    fi

    # Sanity check. At that point we should really be dealing with valid filenames.
    if [[ ! -f "$file" ]]; then
        die "$file is not a file!"
    fi

    echo "$file"
done
