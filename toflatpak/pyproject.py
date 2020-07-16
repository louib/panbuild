#!/usr/bin/env python3
# -*- coding: utf-8 -*-

# Documentation taken from https://python-poetry.org/docs/pyproject/
# The pyproject.toml file
# The tool.poetry section of the pyproject.toml file is composed of multiple sections.
PYPROJECT_MANIFEST_TAGS = [
    # The name of the package. Required
    "name",

    # The version of the package. Required
    # This should follow semantic versioning.
    # However it will not be enforced and you remain free to follow another specification.
    "version",


    # A short description of the package. Required
    "description",

    # The license of the package.
    #
    # The recommended notation for the most common licenses is (alphabetical):
    #
    # Apache-2.0
    # BSD-2-Clause
    # BSD-3-Clause
    # BSD-4-Clause
    # GPL-2.0-only
    # GPL-2.0-or-later
    # GPL-3.0-only
    # GPL-3.0-or-later
    # LGPL-2.1-only
    # LGPL-2.1-or-later
    # LGPL-3.0-only
    # LGPL-3.0-or-later
    # MIT
    # Optional, but it is highly recommended to supply this.
    # More identifiers are listed at the SPDX Open Source License Registry.
    #
    # If your project is proprietary and does not use a specific licence,
    # you can set this value as Proprietary.
    "license",

    # The authors of the package. Required
    # This is a list of authors and should contain at least one author.
    # Authors must be in the form name <email>.
    "authors",

    # The maintainers of the package. Optional
    # This is a list of maintainers and should be distinct from authors.
    # Maintainers may contain an email and be in the form name <email>.
    "maintainers",

    # The readme file of the package. Optional
    # The file can be either README.rst or README.md.
    "readme",

    # An URL to the website of the project. Optional
    "homepage",

    # An URL to the repository of the project. Optional
    "repository",

    # An URL to the documentation of the project. Optional
    "documentation",

    # A list of keywords (max: 5) that the package is related to. Optional
    "keywords",

    # A list of PyPI trove classifiers that describe the project. Optional
    "classifiers",
]


def to_flatpak(pyproject_manifest_path):

    # TODO verify that it's a .toml file?
    return {}
