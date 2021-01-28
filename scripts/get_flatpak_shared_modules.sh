#!/usr/bin/env bash

die() { echo "🔥 Error: $*" 1>&2; exit 1; }

set -e

output_dir="$PB_OUT_DIR"
if [[ -z "$output_dir" ]]; then
    die "You must define the PB_OUT_DIR variable."
fi

if [[ -z "$PROJECTS_DIR" ]]; then
    echo "⚠️ PROJECTS_DIR variable is undefined. Defaulting to current directory!"
    PROJECTS_DIR="."
elif [[ ! -d "$PROJECTS_DIR" ]]; then
    die "$PROJECTS_DIR is not a directory!"
fi

if [[ ! -d "$PROJECTS_DIR/shared-modules" ]]; then
    echo "👍 Did not find shared modules at $PROJECTS_DIR/shared-modules/. Fetching ⬇️!"
    cd "$PROJECTS_DIR";
    git clone https://github.com/flathub/shared-modules.git
else
    echo "👍 No need to fetch the shared modules!"
    cd "$PROJECTS_DIR";
fi

cd shared-modules
mkdir -p "$PB_OUT_DIR"

# sanity check.
if [[ ! -d "$PB_OUT_DIR" ]]; then
    die "$PB_OUT_DIR (the output dir) is not a directory!"
fi

files=$(find "./")
IFS=$'\n'; for file in $files; do
    filename=$(basename "$file")
    if [[ -f "$PB_OUT_DIR/$filename" ]]; then
        echo "🗃️ $PB_OUT_DIR/$filename is already a file in the output directory."
        continue
    fi

    if [[ "$file" == *.json ]]; then
        cp "$file" "$PB_OUT_DIR"
        echo "🗃️ Sending $file to output dir for parsing."
    fi
done
unset IFS
