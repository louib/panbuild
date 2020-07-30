#!/usr/bin/env bash
# Bump the version of the app.

set -e

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
current_version=$(cat "$version_file_path")

# Sanity check.
"./$SCRIPT_DIR/check_version.sh"

# Also sanity check.
git_version=$(git describe --tags --abbrev=0)
if [[ "$git_version" != "$current_version" ]]; then
    die "The git version $git_version is not the same as in the VERSION file!";
fi


# TODO should accept an arg (target version), or default to patch bumping.
echo "Bumping from version $current_version."
# See https://github.com/fsaintjacques/semver-tool/blob/master/src/semver

new_version=$(increment_version_number "$current_version");
echo "New version is $new_version";

sed -i "s/$current_version/$new_version/g" "$version_file_path"

main_file_path="$SCRIPT_DIR/../src/main.rs"
if [[ ! -f "$main_file_path" ]]; then
    die "Could not find main file $main_file_path";
fi
sed -i "s/\"$current_version\"/\"$new_version\"/g" "$main_file_path"

cargo_file_path="$SCRIPT_DIR/../Cargo.toml"
if [[ ! -f "$cargo_file_path" ]]; then
    die "Could not find Cargo file $cargo_file_path";
fi
sed -i "s/version = \"$current_version\"/version = \"$new_version\"/g" "$cargo_file_path"

# This will bump the version in the Cargo.lock file.
cargo install

# TODO add check for version in the man pages!!

# Sanity check.
"./$SCRIPT_DIR/check_version.sh"

git commit -a -n -m "🏷️  $new_version 🏷️"

git tag "$new_version"
git push --tags origin
git push origin master
