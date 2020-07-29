#!/usr/bin/env bash
# Bump the version of the app.

die() { echo "🔥 Error: $*" 1>&2; exit 1; }

SCRIPT_DIR=$(dirname "$0")


version_file_path="$SCRIPT_DIR/../VERSION"
if [[ ! -f "$version_file_path" ]]; then
    die "Could not find version file $version_file_path";
fi
app_version=$(cat "$version_file_path")

# TODO should accept an arg (target version), or default to patch bumping.
echo "Bumping from version $app_version."
# See https://github.com/fsaintjacques/semver-tool/blob/master/src/semver
