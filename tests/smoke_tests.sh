#!/usr/bin/env bash

set -euo pipefail

die() { echo "🔥 Error: $*" 1>&2; exit 1; }

if ! command -v panbuild > /dev/null; then
    die "Could not find the panbuild binary";
fi

echo "🔍 Running smoke tests on the panbuild binary 🔍";

help_output=$(panbuild -h)
if [[ ! "$help_output" == *"The universal build manifest converter."* ]]; then
    die "Missing app description from the help output!";
fi
if [[ ! "$help_output" == *"SUBCOMMANDS"* ]]; then
    die "Missing subcommands from the help output!";
fi
echo "✔️  Validated panbuild -h output";


echo "👍 Smoke tests for panbuild binary ran successfully";
