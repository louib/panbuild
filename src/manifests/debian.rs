use std::collections::HashMap;

// Package constants.
const DEFAULT_ARCH: &str = "any";
const DEFAULT_MULTI_ARCH: &str = "same";

const CONTROL_FILE_SEPARATOR: &str = ":";

//static mut ALLOWED_SECTIONS: Vec<&str> = vec![
//    "admin",
//    "cli-mono",
//    "comm",
//    "database",
//    "debug",
//    "devel",
//    "doc",
//    "editors",
//    "education",
//    "electronics",
//    "embedded",
//    "fonts",
//    "games",
//    "gnome",
//    "gnu-r",
//    "gnustep",
//    "graphics",
//    "hamradio",
//    "haskell",
//    "httpd",
//    "interpreters",
//    "introspection",
//    "java",
//    "javascript",
//    "kde",
//    "kernel",
//    "libdevel",
//    "libs",
//    "lisp",
//    "localization",
//    "mail",
//    "math",
//    "metapackages",
//    "misc",
//    "net",
//    "news",
//    "ocaml",
//    "oldlibs",
//    "otherosfs",
//    "perl",
//    "php",
//    "python",
//    "ruby",
//    "rust",
//    "science",
//    "shells",
//    "sound",
//    "tasks",
//    "tex",
//    "text",
//    "utils",
//    "vcs",
//    "video",
//    "web",
//    "x11",
//    "xfce",
//    "zope",
//];
const DEFAULT_SECTION: &str = "libs";


pub enum Priority {
    // Packages which are necessary for the proper functioning of the system (usually, this means that dpkg functionality depends on these packages).
    // Removing a required package may cause your system to become totally broken and you may not even be able to use dpkg to put things back,
    // so only do so if you know what you are doing.
    //
    // Systems with only the required packages installed have at least enough functionality for the sysadmin to boot the system and install more software.
    required,

    // Important programs, including those which one would expect to find on any Unix-like system.
    // If the expectation is that an experienced Unix person who found it missing would say “What on earth is going on, where is foo?”,
    // it must be an important package. 6 Other packages without which the system will not run well or be usable must also have priority important.
    // This does not include Emacs, the X Window System, TeX or any other large applications.
    // The important packages are just a bare minimum of commonly-expected and necessary tools.
    important,

    // These packages provide a reasonably small but not too limited character-mode system.
    // This is what will be installed by default if the user doesn’t select anything else.
    // It doesn’t include many large applications.
    //
    // No two packages that both have a priority of standard or higher may conflict with each other.
    standard,

    // This is the default priority for the majority of the archive.
    // Unless a package should be installed by default on standard Debian systems,
    // it should have a priority of optional. Packages with a priority of optional may conflict with each other.
    optional,

    // This priority is deprecated. Use the optional priority instead.
    // This priority should be treated as equivalent to optional.
    //
    // The extra priority was previously used for packages that conflicted with other packages and packages
    // that were only likely to be useful to people with specialized requirements. However, this distinction
    // was somewhat arbitrary, not consistently followed, and not useful enough to warrant the maintenance effort.
    extra,
}
const DEFAULT_PRIORITY: Priority = Priority::optional;

// See https://www.debian.org/doc/debian-policy/ch-controlfields.html
struct DebianManifest {
    // The name of the source described in this manifest.
    // (mandatory)
    source: String,

    // The packages in the archive areas main, contrib and non-free are grouped further into sections to simplify handling.
    section: String,

    priority: Priority,

    // Format is name <email@address.com>
    // (mandatory)
    maintainer: String,

    // List of the names and email addresses of co-maintainers of the package, if any.
    // Format is name <email@address.com>
    uploaders: Vec<String>,

    build_depends: Vec<String>,

    // A semver reference to a "canonical" version.
    // (mandatory)
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
            priority: DEFAULT_PRIORITY,
            maintainer: "".to_string(),
            uploaders: vec![],
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

fn parse_paragraphs(content: &str, paragraphs: &mut Vec<String>) {
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

pub fn parse(ctx: &mut crate::execution_context::ExecutionContext) -> i32 {
    let mut paragraphs: Vec<String> = vec![];
    parse_paragraphs(&ctx.content, &mut paragraphs);

    ctx.manifest = crate::manifests::manifest::AbstractManifest::default();

    let mut debian_manifest = DebianManifest::default();
    // TODO validate that there is more than 1 paragraph?
    for paragraph_index in 1..paragraphs.len() {
        let paragraph = &paragraphs[paragraph_index];
        let mut values: HashMap<String, String> = HashMap::new();
        for line in paragraph.split('\n') {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() != 1 {
                eprintln!("Invalid debian control file line {}", line);
                return 1;
            }

            values.insert(parts[0].to_string(), parts[1].to_string());
        }

        let mut package = crate::manifests::manifest::AbstractModule::default();

        package.name = "name".to_string();
        // architecture =
        // multi_arch =
        // depends =
        // description =
        ctx.manifest.modules.push(package);
    }

    eprintln!("finished parsing debian control file.");
    return 0;
}

pub fn dump(ctx: &mut crate::execution_context::ExecutionContext) -> i32 {
    return 0;
}

pub fn is_type(ctx: &mut crate::execution_context::ExecutionContext) -> bool {
    return false;
}
