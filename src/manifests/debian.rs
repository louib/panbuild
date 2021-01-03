use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::manifests::manifest::{AbstractManifest, AbstractModule, Priority};

const CONTROL_FILE_SEPARATOR: &str = ":";

pub const ALLOWED_SECTIONS: [&'static str; 57] = [
    "admin",
    "cli-mono",
    "comm",
    "database",
    "debug",
    "devel",
    "doc",
    "editors",
    "education",
    "electronics",
    "embedded",
    "fonts",
    "games",
    "gnome",
    "gnu-r",
    "gnustep",
    "graphics",
    "hamradio",
    "haskell",
    "httpd",
    "interpreters",
    "introspection",
    "java",
    "javascript",
    "kde",
    "kernel",
    "libdevel",
    "libs",
    "lisp",
    "localization",
    "mail",
    "math",
    "metapackages",
    "misc",
    "net",
    "news",
    "ocaml",
    "oldlibs",
    "otherosfs",
    "perl",
    "php",
    "python",
    "ruby",
    "rust",
    "science",
    "shells",
    "sound",
    "tasks",
    "tex",
    "text",
    "utils",
    "vcs",
    "video",
    "web",
    "x11",
    "xfce",
    "zope",
];

// list of packages (see package fields below).
const PACKAGES: &str = "packages";

// **** Package fields
const PACKAGE_NAME: &str = "Package";
// A multi-line string
const DESCRIPTION: &str = "Description";
// Can be "any"
const ARCHITECTURE: &str = "Architecture";
const MULTI_ARCH: &str = "Multi-Arch";
// These seven fields are used to declare a dependency relationship by one package on another.
// See https://www.debian.org/doc/debian-policy/ch-relationships.html#binary-dependencies-depends-recommends-suggests-enhances-pre-depends
const DEPENDS: &str = "Depends";
const RECOMMENDS: &str = "Recommends";
const SUGGESTS: &str = "Suggests";
const ENHANCES: &str = "Enhances";
const PRE_DEPENDS: &str = "Pre-Depends";
const CONFLICTS: &str = "Conflicts";
const BREAKS: &str = "Breaks";

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct DebianManifest {
    // The name of the source described in this manifest.
    // (mandatory)
    pub source: String,
    pub version: String,
    // The packages in the archive areas main, contrib and non-free are grouped
    // further into sections to simplify handling.
    pub section: String,
    pub priority: String,
    // Format is name <email@address.com>
    // (mandatory)
    pub maintainer: String,
    // List of the names and email addresses of co-maintainers of the package, if any.
    // Format is name <email@address.com>
    pub uploaders: Vec<String>,
    pub build_depends: Vec<String>,
    // A semver reference to a "canonical" version.
    // (mandatory)
    pub standards_version: String,
    // URL of a website to browser the source code.
    pub homepage: String,
    // URL of a website to browser the source code.
    pub vcs_browser: String,
    // URL of the git repo.
    pub vcs_git: String,
}
impl DebianManifest {
    pub fn parse(file_path: &String, manifest_content: &String) -> Option<DebianManifest> {
        // FIXME we need to migrate the function from down below.
        None
    }
}

fn parse_paragraphs(content: &str, paragraphs: &mut Vec<String>) {
    let content_str = content.to_string();
    let lines = content_str.split("\n");
    let mut paragraph: String = String::from("");

    for line in lines {
        if !is_empty_line(line) {
            paragraph.push_str(line);
            paragraph.push_str("\n");
        }
        if is_empty_line(line) && !paragraph.is_empty() {
            paragraphs.push(paragraph);
            paragraph = String::from("");
        }
    }
}

fn is_empty_line(line: &str) -> bool {
    for c in line.chars() {
        if c == ' ' {
            continue;
        }
        if c == '\t' {
            continue;
        }
        return false;
    }
    return true;
}

fn is_indented_line(line: &str) -> bool {
    for c in line.chars() {
        if c == ' ' {
            return true;
        }
        if c == '\t' {
            return true;
        }
        return false;
    }
    return false;
}

fn is_commented_line(line: &str) -> bool {
    for c in line.chars() {
        if c == ' ' {
            continue;
        }
        if c == '\t' {
            continue;
        }
        if c == '#' {
            return true;
        }
        return false;
    }
    return false;
}

pub fn parse(ctx: &mut crate::execution_context::ExecutionContext) {
    let mut paragraphs: Vec<String> = vec![];
    parse_paragraphs(&ctx.content, &mut paragraphs);

    ctx.manifest = crate::manifests::manifest::AbstractManifest::default();
    if paragraphs.len() < 2 {
        panic!("There is only {} paragraphs in the debian control file?", paragraphs.len())
    }

    let mut debian_manifest = DebianManifest::default();

    let first_paragraph = &paragraphs[0];
    for line in first_paragraph.split('\n') {
        if line.starts_with(" ") {
            // Obviously a mistake
            continue;
        }
        if is_empty_line(line) {
            continue;
        }
        if is_commented_line(line) {
            continue;
        }
        let parts: Vec<&str> = line.split(CONTROL_FILE_SEPARATOR).collect();
        if parts.len() < 2 {
            // FIXME we should return a Result<> instead of exiting here or
            // returning a default value.
            eprintln!("Invalid debian control file line {}", line);
            return;
        }
        let field_name = parts[0].trim();

        let value_parts: Vec<&str> = parts[1..].to_vec();
        let mut field_value = value_parts.join(CONTROL_FILE_SEPARATOR);
        field_value = field_value.trim().to_string();

        if field_name == "Source" {
            debian_manifest.source = field_value.to_string();
        }
        if field_name == "Maintainer" {
            debian_manifest.maintainer = field_value.to_string();
        }
        if field_name == "Version" {
            debian_manifest.version = field_value.to_string();
        }
        if field_name == "Priority" {
            debian_manifest.priority = field_value.to_string();
        }
        if field_name == "Standards-Version" {
            debian_manifest.standards_version = field_value.to_string();
        }
        if field_name == "Homepage" {
            debian_manifest.vcs_browser = field_value.to_string();
            debian_manifest.homepage = field_value.to_string();
        }
        if field_name == "Section" {
            debian_manifest.section = field_value.to_string();
            if !ALLOWED_SECTIONS.contains(&&debian_manifest.section[..]) {
                // FIXME we should return a Result<> instead of exiting here or
                // returning a default value.
                eprintln!("Invalid debian control section {}", debian_manifest.section);
                return;
            }
        }
    }

    for paragraph_index in 1..paragraphs.len() {
        let mut package = AbstractModule::default();
        let paragraph = &paragraphs[paragraph_index];
        let mut last_field_name: String = String::from("");

        // Those fields can be multi-line.
        let mut build_depends: String = String::from("");
        let mut description: String = String::from("");

        for line in paragraph.split('\n') {
            if is_commented_line(line) {
                continue;
            }
            if is_empty_line(line) {
                continue;
            }

            let mut field_name: String;
            let mut field_value: String;

            if is_indented_line(line) {
                field_name = last_field_name.clone();
                field_value = line.to_string();
            } else {
                let parts: Vec<&str> = line.split(CONTROL_FILE_SEPARATOR).collect();
                if parts.len() < 2 {
                    // FIXME we should return a Result<> instead of exiting here or
                    // returning a default value.
                    eprintln!("Invalid debian control file line {}", line);
                    return;
                }

                field_name = parts[0].trim().to_string();
                field_value = String::from("");
                for part in parts {
                    if part == field_name {
                        continue;
                    }
                    if !field_value.is_empty() {
                        field_value.push_str(CONTROL_FILE_SEPARATOR);
                    }
                    field_value.push_str(part);
                }
                last_field_name = field_name.clone();
            }

            if field_name == PACKAGE_NAME {
                package.name = field_value.trim().to_string();
            } else if field_name == ARCHITECTURE {
            } else if field_name == DESCRIPTION {
                // Here we append because the field can be multi-line.
                description.push_str(&field_value);
            } else if field_name == DEPENDS {
                build_depends.push_str(&field_value);
            } else {
                eprintln!("Unknown debian package field {}", field_name);
            }
        }

        for package_name in build_depends.split(",") {
            if package_name.is_empty() {
                continue;
            }
            let mut new_module = crate::manifests::manifest::AbstractModule::default();

            let module_spec = package_name.trim().to_string();
            // Still not sure what this is about.
            if module_spec.starts_with("${") {
                continue;
            }

            let mut module_spec_parts: Vec<&str> = module_spec.split(" ").collect();

            new_module.name = module_spec_parts[0].to_string();
            module_spec_parts.remove(0);

            let mut version_spec = String::from("");
            for part in module_spec_parts {
                if !version_spec.is_empty() {
                    version_spec.push_str(" ");
                }
                version_spec.push_str(part);
            }
            new_module.version = version_spec;

            package.depends_on.push(new_module);
        }
        ctx.manifest.depends_on.push(package);
    }

    eprintln!("finished parsing debian control file.");
}

pub fn file_path_matches(path: &str) -> bool {
    if path.to_lowercase().ends_with("debian/control") {
        return true;
    }
    return false;
}

pub fn file_content_matches(content: &str) -> bool {
    return false;
}

const DEBIAN_CONTROL_EXAMPLE: &str = r###"
Source: package_name
Section: x11
Priority: optional
Maintainer: me <me@cloud.com>
Build-Depends:
 debhelper (>= 12),
 gtk-doc-tools,
 libsecret-1-dev,
 libfeedback-dev,
 libgnome-desktop-3-dev,
 libhandy-0.0-dev (>= 0.0.12),
 libpam0g-dev,
# to run the tests
 at-spi2-core,
 gnome-themes-extra-data,
 phoc,
 xauth,
Standards-Version: 3.2.2
Homepage: https://code.cloud.com/projects/package_name

Package: other_package_name
Architecture: any
Depends:
 ${misc:Depends},
 ${shlibs:Depends},
 fonts-lato,
 gsettings-desktop-schemas,
 phoc (>= 0.4.0),
Recommends:
 feedbackd,
 iio-sensor-proxy,
 gnome-session,
 phoc,
Provides:
 notification-daemon,
 polkit-1-auth-agent,
Description: Here's a description of the sub-package
 on multiple lines.

"###;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_is_empty_line() {
        assert_eq!(true, is_empty_line(""));
        assert_eq!(true, is_empty_line("\t\t"));
        assert_eq!(true, is_empty_line("                   "));
        assert_eq!(false, is_empty_line("                   word"));
    }

    #[test]
    pub fn test_is_commented_line() {
        assert_eq!(false, is_commented_line(""));
        assert_eq!(true, is_commented_line("# comment"));
        assert_eq!(true, is_commented_line("         # comment"));
        assert_eq!(false, is_commented_line("Field: Value # with a comment after."));
    }

    #[test]
    pub fn test_is_indented_line() {
        assert_eq!(false, is_indented_line(""));
        assert_eq!(true, is_indented_line("  line"));
        assert_eq!(true, is_indented_line("  comment    "));
        assert_eq!(false, is_indented_line("Field: Value # with a comment after."));
    }

    #[test]
    pub fn test_parse() {
        let mut ctx = crate::execution_context::ExecutionContext::default();
        ctx.content = DEBIAN_CONTROL_EXAMPLE.to_string();
        parse(&mut ctx);
        match ctx.manifest.debian_manifest {
            None => panic!("Error while parsing the debian manifest."),
            Some(manifest) => {
                assert!(manifest.source == "package_name", "The app name was not package_name!",);
                assert!(manifest.vcs_browser == "https://code.cloud.com/projects/package_name");
            }
        }
    }

    #[test]
    pub fn test_file_path_matches() {
        assert!(file_path_matches("debian/control"));
        assert!(file_path_matches("path/to/the/debian/control"));
        assert!(file_path_matches("the/Debian/CONTROL"));
        assert!(!file_path_matches("control"));
        assert!(!file_path_matches(".flatpak-builder/cache/objects/54/.file/user/1000/keyring/control"));
        assert!(!file_path_matches("/path/to/file.yaml"));
        assert!(!file_path_matches("/path/to/file.json"));
        assert!(!file_path_matches(""));
        assert!(!file_path_matches("/////////////"));
    }
}
