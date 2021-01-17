use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::manifests::manifest::{AbstractManifest, AbstractModule};

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

pub const ALLOWED_PACKAGE_FIELDS: [&'static str; 11] = [
    // The name of the package being described.
    "Package",
    // A multi-line description.
    "Description",
    // Can be "any"
    "Architecture",
    "Multi-Arch",
    // These seven fields are used to declare a dependency relationship by one package on another.
    // See https://www.debian.org/doc/debian-policy/ch-relationships.html#binary-dependencies-depends-recommends-suggests-enhances-pre-depends
    "Depends",
    "Recommends",
    "Suggests",
    "Enhances",
    "Pre-Depends",
    "Conflicts",
    "Breaks",
];

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
    pub fn parse(manifest_content: &String) -> Option<DebianManifest> {
        let paragraphs = parse_paragraphs(manifest_content);
        if paragraphs.len() < 2 {
            eprintln!("There is only {} paragraph in the debian control file?", paragraphs.len());
            return None;
        }

        let mut debian_manifest = DebianManifest::default();

        let first_paragraph = parse_paragraph(&paragraphs[0]);
        debian_manifest.source = first_paragraph.get("Source").unwrap_or(&"".to_string()).to_string();
        debian_manifest.maintainer = first_paragraph.get("Maintainer").unwrap_or(&"".to_string()).to_string();
        debian_manifest.version = first_paragraph.get("Version").unwrap_or(&"".to_string()).to_string();
        debian_manifest.priority = first_paragraph.get("Priority").unwrap_or(&"".to_string()).to_string();
        debian_manifest.standards_version = first_paragraph.get("Standards-Version").unwrap_or(&"".to_string()).to_string();
        debian_manifest.vcs_browser = first_paragraph.get("Homepage").unwrap_or(&"".to_string()).to_string();

        let build_depends = first_paragraph.get("Build-Depends").unwrap_or(&"".to_string()).to_string();
        for dependency in build_depends.split(',') {
            if dependency.is_empty() {
                continue;
            }
            // We ignore documentation related dependencies.
            if dependency.contains("<!nodoc>") {
                continue;
            }
            // Known build tools.
            if dependency.contains("meson") {
                continue;
            }

            debian_manifest.build_depends.push(dependency.trim().to_string());
        }

        debian_manifest.homepage = first_paragraph.get("Homepage").unwrap_or(&"".to_string()).to_string();
        debian_manifest.section = first_paragraph.get("Section").unwrap_or(&"".to_string()).to_string();
        if !ALLOWED_SECTIONS.contains(&&debian_manifest.section[..]) {
            eprintln!("Invalid debian control section {}", debian_manifest.section);
            return None;
        }

        // TODO parse the other paragraphs when we support executables.
        Some(debian_manifest)
    }
}

fn parse_paragraphs(content: &String) -> Vec<String> {
    let mut paragraphs = vec![];
    let lines = content.split("\n");
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
    if !paragraph.is_empty() {
        paragraphs.push(paragraph);
    }
    paragraphs
}

fn parse_paragraph(paragraph: &String) -> HashMap<String, String> {
    let mut fields: HashMap<String, String> = HashMap::new();
    let lines = paragraph.split("\n");

    let mut field_name: String = String::from("");
    let mut field_value: String = String::from("");

    for line in lines {
        if is_empty_line(line) {
            continue;
        }
        if is_commented_line(line) {
            continue;
        }
        if is_field_start(line) {
            if !field_name.is_empty() {
                fields.insert(field_name, field_value.trim().to_string());
            }

            let parts: Vec<&str> = line.split(CONTROL_FILE_SEPARATOR).collect();

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
        } else if field_name.is_empty() {
            continue;
        } else {
            field_value += line;
        }
    }

    if !field_name.is_empty() {
        fields.insert(field_name, field_value.trim().to_string());
    }
    fields
}

fn is_field_start(line: &str) -> bool {
    for c in line.chars() {
        if c.is_alphanumeric() {
            continue;
        }
        if c == '-' {
            continue;
        }
        if c == CONTROL_FILE_SEPARATOR.chars().nth(0).unwrap() {
            return true;
        }
        return false;
    }
    return true;
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

pub fn file_path_matches(path: &str) -> bool {
    if path.to_lowercase().ends_with("debian/control") {
        return true;
    }
    return false;
}

// Currently the documentation comes from the Debian control file documentation.
pub enum Priority {
    // Packages which are necessary for the proper functioning of the system (usually,
    // this means that dpkg functionality depends on these packages).
    // Removing a required package may cause your system to become totally broken and you
    // may not even be able to use dpkg to put things back,
    // so only do so if you know what you are doing.
    //
    // Systems with only the required packages installed have at least enough functionality for
    // the sysadmin to boot the system and install more software.
    Required,

    // Important programs, including those which one would expect to find on any Unix-like system.
    // If the expectation is that an experienced Unix person who found it missing would
    // say “What on earth is going on, where is foo?”,
    // it must be an important package. 6 Other packages without which the system will not run
    // well or be usable must also have priority important.
    // This does not include Emacs, the X Window System, TeX or any other large applications.
    // The important packages are just a bare minimum of commonly-expected and necessary tools.
    Important,

    // These packages provide a reasonably small but not too limited character-mode system.
    // This is what will be installed by default if the user doesn’t select anything else.
    // It doesn’t include many large applications.
    //
    // No two packages that both have a priority of standard or higher may conflict with each other.
    Standard,

    // This is the default priority for the majority of the archive.
    // Unless a package should be installed by default on standard Debian systems,
    // it should have a priority of optional. Packages with a priority of optional
    // may conflict with each other.
    Optional,

    // This priority is deprecated. Use the optional priority instead.
    // This priority should be treated as equivalent to optional.
    //
    // The extra priority was previously used for packages that conflicted with other packages and packages
    // that were only likely to be useful to people with specialized requirements. However, this distinction
    // was somewhat arbitrary, not consistently followed, and not useful enough to warrant
    // the maintenance effort.
    Extra,
}

const DEFAULT_PRIORITY: Priority = Priority::Optional;

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
    pub fn test_parse() {
        match DebianManifest::parse(&DEBIAN_CONTROL_EXAMPLE.to_string()) {
            None => panic!("Error while parsing the debian manifest."),
            Some(manifest) => {
                assert!(manifest.source == "package_name", "The app name was not package_name!",);
                assert!(manifest.vcs_browser == "https://code.cloud.com/projects/package_name");
                assert_ne!(manifest.build_depends.len(), 0);
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
