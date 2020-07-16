#!/usr/bin/env python3
# -*- coding: utf-8 -*-
from os import listdir, system
from os.path import isfile, join

# FIXME fix that import.
# from .toflatpak.toflatpak import snap_to_flatpak

current_script_path = __file__

# FIXME make this relative to the current script.
FIXTURES_DIR = "tests/fixtures"
OUTPUT_DIR = "tests/output"

# Cleaning the output dir.
system("mkdir -p {0}".format(OUTPUT_DIR))
if len(listdir(OUTPUT_DIR)):
    system("rm {0}/*".format(OUTPUT_DIR))

print("🔍 Starting functional test suite for 2flatpak.")

if __name__ == '__main__':
    for fixtures_file in listdir(FIXTURES_DIR):

        path = join(FIXTURES_DIR, fixtures_file)
        # sanity check, we should be dealing with files at that point.
        if not isfile(path):
            continue

        if not fixtures_file.endswith('.yaml'):
            continue

        test_case_name = fixtures_file[-4]
        fixture_body = open(path, 'r')

        # flatpak_manifest = snap_to_flatpak(fixture_body)
        # print(flatpak_manifest)
