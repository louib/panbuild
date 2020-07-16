#!/usr/bin/env python3
from os import listdir, system
from os.path import isfile, join


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

        # sanity check, we should be dealing with files at that point.
        if not isfile(join(FIXTURES_DIR, fixtures_file)):
            continue

        if not fixtures_file.endswith('.yaml'):
            continue
