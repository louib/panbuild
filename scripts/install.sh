#!/usr/bin/env bash

die() { echo "🔥 Error: $*" 1>&2; exit 1; }

if [[ -z "$PANBUILD_INSTALL_DIR" ]]; then
    die "Please specify the PANBUILD_INSTALL_DIR";

fi

cargo build --bins --release
cargo install --path .
mkdir -p "$PANBUILD_INSTALL_DIR"
install ./target/release/panbuild "$PANBUILD_INSTALL_DIR"

export PATH="$PATH:$PANBUILD_INSTALL_DIR"
