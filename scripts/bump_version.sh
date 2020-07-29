#!/usr/bin/env bash
# Bump the version of the app.

die() { echo "🔥 Error: $*" 1>&2; exit 1; }

SCRIPT_DIR=$(dirname "$0")

if [[ -n $(git status -s) ]]; then
  git status -s
  die "Your git environment needs to be clean before bumping the version!"
fi

branch_name="$(git symbolic-ref HEAD 2>/dev/null)"
branch_name=${branch_name##refs/heads/}
if [[ ! -z "$branch_name" && "$branch_name" != "master" ]]; then
    die "You should be on the master branch!";
fi
echo "on $branch_name"

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

new_version=$(increment_version_number "$app_version");
echo "New version is $new_version";

main_file_path="$SCRIPT_DIR/../src/main.rs"
if [[ ! -f "$main_file_path" ]]; then
    die "Could not find main file $main_file_path";
fi
sed -i "s/\"$current_version\"/\"$new_version\"/g" "$main_file_path"

# git commit -a -n -m "🏷️ $new_version 🏷️"

# git tag "$new_version"
# git push --tags origin
# git push origin master

# Sanity check.
"./$SCRIPT_DIR/check_version.sh"
