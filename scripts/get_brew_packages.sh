#!/usr/bin/env bash
# Get the brew packages.

die() { echo "🔥 Error: $*" 1>&2; exit 1; }

set -e

output_dir="$PB_OUT_DIR"
if [[ -z "$output_dir" ]]; then
    die "You must define the PB_OUT_DIR variable."
fi

if [[ ! -d "$output_dir" ]]; then
    die "$output_dir is not a directory!"
fi

if [[ ! -f "$PB_OUT_DIR/formula-linux.json" ]]; then
    # All the formulae for macOS
    # wget https://formulae.brew.sh/api/formula.json

    # All the formulae for Linux
    wget https://formulae.brew.sh/api/formula-linux.json
    mv formula-linux.json "$PB_OUT_DIR"

    # All the casks
    # wget https://formulae.brew.sh/api/cask.json

    echo "👍 Fetched sources from brew recipes."
else
    echo "👍 No need to fetch brew recipes."
fi
