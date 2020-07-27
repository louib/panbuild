#!/usr/bin/env bash

set -uo pipefail

die() { echo "🔥 Error: $*" 1>&2; exit 1; }

if ! command -v panbuild > /dev/null; then
    die "Could not find the panbuild binary";
fi

echo "🔍 Running smoke tests on the panbuild binary 🔍";

help_output=$(panbuild -h 2>&1)
if [[ ! "$help_output" == *"The universal build manifest converter."* ]]; then
    die "Missing app description from the help output!";
fi
if [[ ! "$help_output" == *"SUBCOMMANDS"* ]]; then
    die "Missing subcommands description from the help output!";
fi
echo "✔️  Validated panbuild -h output";

no_command_output=$(panbuild 2>&1)
if [[ ! "$no_command_output" == *"Please provide a command to execute"* ]]; then
    die "Missing error message when no command provided!";
fi
if [[ ! "$no_command_output" == *"USAGE"* ]]; then
    die "Missing USAGE text when no command provided!";
fi
if [[ ! "$no_command_output" == *"SUBCOMMAND"* ]]; then
    die "Missing SUBCOMMAND text when no command provided!";
fi
echo "✔️  Validated panbuild output when no command provided.";


echo "👍 Smoke tests for panbuild binary ran successfully";
