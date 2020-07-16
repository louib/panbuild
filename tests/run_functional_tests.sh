#!/usr/bin/env bash

SCRIPT_DIR=$(realpath "$0")
SCRIPT_DIR=$(dirname "$SCRIPT_DIR")

FIXTURES_DIR="$SCRIPT_DIR/fixtures/"
OUTPUT_DIR="$SCRIPT_DIR/output/"

# Cleaning the output dir.
mkdir -p "$OUTPUT_DIR"
rm "$OUTPUT_DIR/*"

echo "🔍 Starting functional test suite for 2flatpak."
fixtures_files=$(find "$FIXTURES_DIR" | sort)

IFS=$'\n'; for file in $fixtures_files; do
    if [[ -d "$file" ]]; then
        continue
    fi

    # Sanity check. At that point we should really be dealing with valid filenames.
    if [[ ! -f "$file" ]]; then
        die "$file is not a file!"
    fi

    test_file_name=

    output_file_path="$OUTPUT_DIR"
    if [[ "$file" =~ .*\.yml$ ]]; then 
        echo "$file is a yaml file!!"
        coverage run "$SCRIPT_DIR/../src/snap2flatpak" "$file" > "lol.yaml"
    fi
done
