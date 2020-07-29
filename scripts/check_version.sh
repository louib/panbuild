#!/usr/bin/env bash
# Check that the multiple instances of the app version are in sync.

die() { echo "🔥 Error: $*" 1>&2; exit 1; }

SCRIPT_DIR=$(dirname "$0")


version_file_path="$SCRIPT_DIR/../VERSION"
if [[ ! -f "$version_file_path" ]]; then
    die "Could not find version file $version_file_path";
fi
app_version=$(cat "$version_file_path")

echo "Verifying that version is $app_version everywhere."

main_file_path="$SCRIPT_DIR/../src/main.rs"
if [[ ! -f "$main_file_path" ]]; then
    die "Could not find main file $main_file_path";
fi
is_in_main=$(grep -E "const APP_VERSION: &str = \"$app_version\"" "$main_file_path")
if [[ -z "$is_in_main" ]]; then
    die "Application version $app_version not found in main file $main_file_path";
fi

echo "✔️  Version is $app_version everywhere!";
