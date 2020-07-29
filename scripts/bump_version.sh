#!/usr/bin/env bash
# Bump the version of the app.

die() { echo "🔥 Error: $*" 1>&2; exit 1; }

SCRIPT_DIR=$(dirname "$0")

# Sanity check.
"./$SCRIPT_DIR/check_version.sh"

function increment_version_number () {

    local regex="([0-9]+)\\.([0-9]+)\\.([0-9]+)"
    local current_version=$1
    while [[ $current_version =~ $regex ]]; do
        # Index 0 is always the original full string.
        local major="${BASH_REMATCH[1]}"
        local minor="${BASH_REMATCH[2]}"
        local patch="${BASH_REMATCH[3]}"

        # FIXME we should allow to bump minors and majors too.
        local patch=$((patch + 1));
        echo "$major.$minor.$patch";
        break;
    done
}

version_file_path="$SCRIPT_DIR/../VERSION"
if [[ ! -f "$version_file_path" ]]; then
    die "Could not find version file $version_file_path";
fi
app_version=$(cat "$version_file_path")

# TODO should accept an arg (target version), or default to patch bumping.
echo "Bumping from version $app_version."
# See https://github.com/fsaintjacques/semver-tool/blob/master/src/semver

new_version_number=$(increment_version_number "$app_version");
echo "New version is $new_version_number";

# Sanity check.
"./$SCRIPT_DIR/check_version.sh"
