// Debian control file constants.
const DEFAULT_SECTION: &str = "libs";
const DEFAULT_PRIORITY: &str = "optional";

// Package constants.
const DEFAULT_ARCH: &str = "any";
const DEFAULT_MULTI_ARCH: &str = "same";

// See https://www.debian.org/doc/debian-policy/ch-controlfields.html
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

pub fn parse(ctx: &crate::execution_context::ExecutionContext) -> i32 {
    let lines = ctx.content.split("\n");
    // let mut paragraphs = Vec<Vec<String>>;
    let mut count = 0;
    for line in lines {
        eprintln!("***** {}", line);
        let mut only_spaces = true;
        let mut indent_size = 0;
        let is_empty_line: bool = line.starts_with(|c: char| {
            if c == ' ' {
                indent_size = indent_size + 1;
                return true;
            }
            if c == '\t' {
                return true;
            }
            return false;
        });
        count = count + 1;
    }
    eprintln!("***** finished parsing debian control file.");
    return 0;
}

pub fn dump(ctx: &crate::execution_context::ExecutionContext) -> i32 {
    return 0;
}

pub fn is_type(ctx: &crate::execution_context::ExecutionContext) -> bool {
    return false;
}
