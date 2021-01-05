#!/usr/bin/env bash
# Get all the projects from the different sources and merges them
# into a single projects db.

die() { echo "🔥 Error: $*" 1>&2; exit 1; }

set -e

SCRIPT_DIR=$(dirname "$0")

if [[ -z "$PB_PROJECTS_DB_PATH" ]]; then
    die "Must define PB_PROJECTS_DB_PATH.";
fi
if [[ ! -d "$PB_PROJECTS_DB_PATH" ]]; then
    die "$PB_PROJECTS_DB_PATH is not a directory.";
fi

tmp_dir=$(mktemp -d -t panbuild-XXXXXXXXXX)

mkdir -p "$tmp_dir/projects"

PB_OUT_DIR="$tmp_dir" bash "$SCRIPT_DIR/get_debian_source_packages.sh"
cat "$tmp_dir/sources.txt" | python3 "$SCRIPT_DIR/parse_debian_source_packages.py" > "$tmp_dir/projects/debian_projects.json"

PB_OUT_DIR="$tmp_dir" bash "$SCRIPT_DIR/get_brew_packages.sh"
cat "$tmp_dir/formula-linux.json" | python3 "$SCRIPT_DIR/parse_brew_packages.py" > "$tmp_dir/projects/brew_projects.json"

PB_OUT_DIR="$tmp_dir/projects/" python3 "$SCRIPT_DIR/merge_projects.py"
mv "$tmp_dir/projects/all_projects.json" "$PB_PROJECTS_DB_PATH"
