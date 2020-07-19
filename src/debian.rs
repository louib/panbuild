// Debian control file constants.
const DEFAULT_SECTION: &str = "libs";
const DEFAULT_PRIORITY: &str = "optional";

// Package constants.
const DEFAULT_ARCH: &str = "any";
const DEFAULT_MULTI_ARCH: &str = "same";

struct DebianManifest {
    // The name of the source described in this manifest.
    source: String,

    // Can be:
    //   * libs
    section: String,

    // Can be:
    //   * optional
    priority: String,

    // Format is name <email@address.com>
    maintainer: String,

    build_depends: Vec<String>,

    // A semver reference to a "canonical" version.
    standards_versions: String,

    homepage: String,

    // URL of a website to browser the source code.
    vcs_browser: String,

    // URL of the git repo.
    vcs_git: String,

    packages: Vec<Package>,
}

struct Package {
    name: String,

    // Can be "any"
    architecture: String,

    multi_arch: String,

    depends: Vec<String>,

    // A multi-line string
    description: String,
}
