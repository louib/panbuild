#!/usr/bin/env bash
# Publish a new version the app.

set -e

die() { echo "🔥 Error: $*" 1>&2; exit 1; }

SCRIPT_DIR=$(dirname "$0")

if ! command -v flatpak; then
    die "Missing flatpak";
fi

if ! command -v flatpak-builder; then
    die "Missing flatpak-builder";
fi

if ! command -v cargo; then
    die "Missing cargo";
fi

# Sanity check.
"./$SCRIPT_DIR/check_version.sh"

cargo build --release
cargo publish
echo "📦 Published the package on crates.io."

# TODO publish on flathub.
# flatpak remote-add --user --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
# flatpak install --user -y flathub org.gnome.Platform/x86_64/3.36 org.gnome.Sdk/x86_64/3.36
# flatpak-builder --force-clean --user build net.louib.panbuild.json
# flatpak-builder --user --run build net.louib.panbuild.json panbuild -V
# echo "📦 Published the package on flathub.com."

# Sanity check.
"./$SCRIPT_DIR/check_version.sh"
