#!/usr/bin/env bash

if [[ ! command -v panbuild ]]; then

fi

# FIXME make this relative to the current script.
FIXTURES_DIR="tests/fixtures"
OUTPUT_DIR="tests/output"

# Cleaning the output dir.
mkdir -p "$OUTPUT_DIR"
rm "{0}/*"

echo "🔍 Starting functional test suite for 2flatpak."

for fixtures_file in listdir(FIXTURES_DIR):

    path = join(FIXTURES_DIR, fixtures_file)
    # sanity check, we should be dealing with files at that point.
    if not isfile(path):
        continue

    if not fixtures_file.endswith('.yaml'):
        continue

    test_case_name = fixtures_file[-4]
    fixture_body = open(path, 'r')

    flatpak_manifest = snap.to_flatpak(fixture_body)
    print(flatpak_manifest)
