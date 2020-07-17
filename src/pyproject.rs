// Documentation taken from https://python-poetry.org/docs/pyproject/
// The pyproject.toml file
// The tool.poetry section of the pyproject.toml file is composed of multiple sections.
struct PyProjectManifest {
    // The name of the package. Required
    name: String,

    // The version of the package. Required
    // This should follow semantic versioning.
    // However it will not be enforced and you remain free to follow another specification.
    version: String,

    // A short description of the package. Required
    description: String,

    // The license of the package.
    //
    // The recommended notation for the most common licenses is (alphabetical):
    //
    // Apache-2.0
    // BSD-2-Clause
    // BSD-3-Clause
    // BSD-4-Clause
    // GPL-2.0-only
    // GPL-2.0-or-later
    // GPL-3.0-only
    // GPL-3.0-or-later
    // LGPL-2.1-only
    // LGPL-2.1-or-later
    // LGPL-3.0-only
    // LGPL-3.0-or-later
    // MIT
    // Optional, but it is highly recommended to supply this.
    // More identifiers are listed at the SPDX Open Source License Registry.
    //
    // If your project is proprietary and does not use a specific licence,
    // you can set this value as Proprietary.
    license: String,

    // The authors of the package. Required
    // This is a list of authors and should contain at least one author.
    // Authors must be in the form name <email>.
    authors: [String;5],

    // The maintainers of the package. Optional
    // This is a list of maintainers and should be distinct from authors.
    // Maintainers may contain an email and be in the form name <email>.
    maintainers: [String;5],

    // The readme file of the package. Optional
    // The file can be either README.rst or README.md.
    readme: String,

    // An URL to the website of the project. Optional
    homepage: String,

    // An URL to the repository of the project. Optional
    repository: String,

    // An URL to the documentation of the project. Optional
    documentation: String,

    // A list of keywords (max: 5) that the package is related to. Optional
    keywords: String,

    // A list of PyPI trove classifiers that describe the project. Optional
    classifiers: String,
}
