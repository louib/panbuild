#!/usr/bin/env bash
# Get the packages of type "source" from the most common
# debian and debian derivative repositories.

die() { echo "🔥 Error: $*" 1>&2; exit 1; }

SCRIPT_DIR=$(dirname "$0")

set -e

wget --no-check-certificate http://us.archive.ubuntu.com/ubuntu/dists/devel/main/source/Sources.gz
gzip -d Sources.gz
mv Sources ubuntu_devel_main_sources.txt

wget --no-check-certificate http://us.archive.ubuntu.com/ubuntu/dists/devel/universe/source/Sources.gz
gzip -d Sources.gz
mv Sources ubuntu_devel_universe_sources.txt

wget --no-check-certificate http://us.archive.ubuntu.com/ubuntu/dists/devel/multiverse/source/Sources.gz
gzip -d Sources.gz
mv Sources ubuntu_devel_multiverse_sources.txt

wget --no-check-certificate https://ftp.nl.debian.org/debian/dists/sid/main/source/Sources.gz
gzip -d Sources.gz
mv Sources debian_sid_main_sources.txt

wget --no-check-certificate https://repo.pureos.net/pureos/dists/green/main/source/Sources.xz
unxz -d Sources.xz
mv Sources pureos_green_main_sources.txt

wget --no-check-certificate https://repo.pureos.net/pureos/dists/landing/main/source/Sources.xz
unxz -d Sources.xz
mv Sources pureos_landing_main_sources.txt

wget --no-check-certificate https://repo.pureos.net/pureos/dists/amber/main/source/Sources.xz
unxz -d Sources.xz
mv Sources pureos_amber_main_sources.txt

wget --no-check-certificate https://repo.pureos.net/pureos/dists/byzantium/main/source/Sources.xz
unxz -d Sources.xz
mv Sources pureos_byzantium_main_sources.txt

cat *_sources.txt > sources.txt
