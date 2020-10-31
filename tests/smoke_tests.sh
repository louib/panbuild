#!/usr/bin/env bash

set -uo pipefail

die() { echo "🔥 Error: $*" 1>&2; exit 1; }

SCRIPT_DIR=$(dirname "$0")

if ! command -v panbuild > /dev/null; then
    die "Could not find the panbuild binary";
fi

echo "🔍 Running smoke tests on the panbuild binary 🔍";

help_output=$(panbuild -h 2>&1)
if [[ ! "$help_output" == *"The universal builder."* ]]; then
    die "Missing app description from the help output!";
fi
if [[ ! "$help_output" == *"SUBCOMMANDS"* ]]; then
    die "Missing subcommands description from the help output!";
fi
echo "✔️  Validated panbuild -h output";

version_output=$(panbuild -V 2>&1)
version_file_path="$SCRIPT_DIR/../VERSION"
if [[ ! -f "$version_file_path" ]]; then
    die "Could not find version file $version_file_path";
fi
expected_version=$(cat "$version_file_path")
if [[ ! "$version_output" == *"$expected_version"* ]]; then
    die "Invalid app version from the version output!";
fi
echo "✔️  Validated panbuild -V output";

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

get_package_missing_args=$(panbuild get-package-list 2>&1)
if [[ ! "$get_package_missing_args" == *"arguments were not provided"* ]]; then
    die "Unexpected error message when calling get-package-list without arguments!";
fi

invalid_command_name_output=$(panbuild not-a-command 2>&1)
if [[ ! "$invalid_command_name_output" == *"isn't valid in this context"* ]]; then
    die "Unexpected error message when providing an invalid command name!";
fi

echo "👍 Smoke tests for panbuild binary ran successfully";
