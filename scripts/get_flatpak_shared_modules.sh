#!/usr/bin/env bash
# Get the packages of type "source" from the most common
# debian and debian derivative repositories.

die() { echo "🔥 Error: $*" 1>&2; exit 1; }

set -e

output_dir="$SHARED_MODULES_OUT_DIR"
if [[ -z "$output_dir" ]]; then
    die "You must define the SHARED_MODULES_OUT_DIR variable."
fi

projects_dir="$PROJECTS_DIR"
if [[ -z "$projects_dir" ]]; then
    echo "⚠️ PROJECTS_DIR variable is undefined. Defaulting to current directory!"
    projects_dir="."
elif [[ ! -d "$projects_dir" ]]; then
    die "$PROJECTS_DIR is not a directory!"
fi

if [[ ! -d "$projects_dir/shared-modules" ]]; then
    echo "👍 Did not find shared modules at ./shared-modules/. Fetching ⬇️!"
    git clone https://github.com/flathub/shared-modules.git
else
    echo "👍 No need to fetch the shared modules!"
fi

cd shared-modules
mkdir -p "$SHARED_MODULES_OUT_DIR"

# sanity check.
if [[ ! -d "$SHARED_MODULES_OUT_DIR" ]]; then
    die "$SHARED_MODULES_OUT_DIR (the output dir) is not a directory!"
fi

files=$(find "./")
IFS=$'\n'; for file in $files; do
    if [[ "$file" == *.json ]]; then
        cp "$file" "SHARED_MODULES_OUT_DIR"
        echo "🗃️ Sending $file to output dir for parsing."
    fi
done
unset IFS
