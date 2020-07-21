use std::collections::HashMap;

use crate::manifests::manifest::{Priority, AbstractModule, AbstractManifest};

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

}

// list of packages.
const packages: &str = "packages";

struct DebianPackage {
    name: String,

    // Can be "any"
    architecture: String,

    multi_arch: String,

    depends: Vec<String>,

    // A multi-line string
    description: String,
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

    ctx.manifest = AbstractManifest::default();

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

        let mut package = AbstractModule::default();

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
