// Debian control file constants.
const DEFAULT_SECTION: &str = "libs";
const DEFAULT_PRIORITY: &str = "optional";

// Package constants.
const DEFAULT_ARCH: &str = "any";
const DEFAULT_MULTI_ARCH: &str = "same";

const CONTROL_FILE_SEPARATOR: &str = ":";

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

    packages: Vec<DebianPackage>,
}

impl Default for DebianManifest {
    fn default() -> Self {
        return DebianManifest {
            source: "".to_string(),
            section: DEFAULT_SECTION.to_string(),
            priority: DEFAULT_PRIORITY.to_string(),
            maintainer: "".to_string(),
            build_depends: vec![],
            standards_versions: "".to_string(),
            homepage: "".to_string(),
            vcs_browser: "".to_string(),
            vcs_git: "".to_string(),
            packages: vec![],
        };
    }
}

struct DebianPackage {
    name: String,

    // Can be "any"
    architecture: String,

    multi_arch: String,

    depends: Vec<String>,

    // A multi-line string
    description: String,
}

impl Default for DebianPackage {
    fn default() -> Self {
        return DebianPackage {
            name: "".to_string(),
            architecture: DEFAULT_ARCH.to_string(),
            multi_arch: DEFAULT_MULTI_ARCH.to_string(),
            description: "".to_string(),
            depends: vec![],
        };
    }
}

fn read_manifest_paragraph(paragraph: String, manifest: &DebianManifest) {

}

fn read_package_paragraph(paragraph: String, package: &DebianPackage) {

}

fn parse_paragraphs(content: &str, mut paragraphs: Vec<String>) {
    let content_str = content.to_string();
    let lines = content_str.split("\n");
    let mut paragraph: String = String::from("");

    for line in lines {
        let mut only_spaces = true;
        let mut indent_size = 0;
        line.starts_with(|c: char| {
            if c == ' ' {
                indent_size = indent_size + 1;
                return true;
            }
            if c == '\t' {
                indent_size = indent_size + 1;
                return true;
            }
            return false;
        });
        let is_empty_line: bool = indent_size == line.len();
        if ! is_empty_line {
            paragraph.push_str(line);
            paragraph.push_str("\n");
        }
        if is_empty_line && ! paragraph.is_empty() {
            paragraphs.push(paragraph);
            paragraph = String::from("");
        }
    }

    eprintln!("***** there was {} paragraphs.", paragraphs.len());
    for paragraph in paragraphs {
        eprintln!("***** paragraph is: {}\n", paragraph);
    }

}

pub fn parse(ctx: &crate::execution_context::ExecutionContext) -> i32 {
    let mut paragraphs: Vec<String> = vec![];
    parse_paragraphs(&ctx.content, paragraphs);

    let mut debian_manifest = DebianManifest::default();
    let mut debian_package = DebianPackage::default();
    let mut packages: Vec<DebianPackage> = vec![];

    // TODO validate that there is more than 1 paragraph?

    eprintln!("finished parsing debian control file.");
    return 0;
}

pub fn dump(ctx: &crate::execution_context::ExecutionContext) -> i32 {
    return 0;
}

pub fn is_type(ctx: &crate::execution_context::ExecutionContext) -> bool {
    return false;
}
