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


// See https://www.debian.org/doc/debian-policy/ch-controlfields.html
// **** Top-level fields
//
// The name of the source described in this manifest.
// (mandatory)
// string
const SOURCE: &str = "Source";
// The packages in the archive areas main, contrib and non-free are grouped further into sections to simplify handling.
// string
const SECTION: &str = "Section";
// The priority
const PRIORITY: &str = "Priority";
// Format is name <email@address.com>
// (mandatory)
// string
const MAINTAINER: &str = "Maintainer";
// List of the names and email addresses of co-maintainers of the package, if any.
// Format is name <email@address.com>
// list of strings
const UPLOADERS: &str = "Uploaders";
// list of strings
const BUILD_DEPENDS: &str = "Build-Depends";
// A semver reference to a "canonical" version.
// (mandatory)
// string
const STANDARDS_VERSIONS: &str = "Standards-Versions";
// string
const HOMEPAGE: &str = "Homepage";
// URL of a website to browser the source code.
// string
const VCS_BROWSER: &str = "Vcs-Browser";
// URL of the git repo.
// string
const VCS_GIT: &str = "Vcs-Git";
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


fn parse_paragraphs(content: &str, paragraphs: &mut Vec<String>) {
    let content_str = content.to_string();
    let lines = content_str.split("\n");
    let mut paragraph: String = String::from("");

    for line in lines {
        if ! is_empty_line(line) {
            paragraph.push_str(line);
            paragraph.push_str("\n");
        }
        if is_empty_line(line) && ! paragraph.is_empty() {
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

pub fn parse(content: &str) -> crate::manifests::manifest::AbstractManifest {
    let mut paragraphs: Vec<String> = vec![];
    parse_paragraphs(&content, &mut paragraphs);

    let mut response = crate::manifests::manifest::AbstractManifest::default();

    if paragraphs.len() < 2 {
        panic!("There is only {} paragraphs in the debian control file?", paragraphs.len())
    }

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
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() < 2 {
            // FIXME we should return a Result<> instead of exiting here or
            // returning a default value.
            eprintln!("Invalid debian control file line {}", line);
            return response;
        }
        let field_name = parts[0].trim();
        let field_value = parts[1].trim();

        if field_name == SOURCE {
            response.package_name = field_value.to_string();
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
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() < 2 {
                    // FIXME we should return a Result<> instead of exiting here or
                    // returning a default value.
                    eprintln!("Invalid debian control file line {}", line);
                    return response;
                }

                field_name = parts[0].trim().to_string();
                field_value = String::from("");
                for part in parts {
                    if part == field_name {
                        continue;
                    }
                    if ! field_value.is_empty() {
                        field_value.push_str(":");
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
                if ! version_spec.is_empty() {
                    version_spec.push_str(" ");
                }
                version_spec.push_str(part);
            }
            new_module.version = version_spec;

            package.depends_on.push(new_module);
        }
        response.depends_on.push(package);
    }

    eprintln!("finished parsing debian control file.");
    return response;
}

pub fn dump(manifest: &crate::manifests::manifest::AbstractManifest) -> String {
    let mut response = String::from("");
    response.push_str(&format!("Source: {}", manifest.package_name));
    return response;
}

pub fn file_path_matches(path: &str) -> bool {
    if path.ends_with("control") {
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
        let mut manifest = parse(&DEBIAN_CONTROL_EXAMPLE);
        assert!(
            manifest.package_name == "package_name",
            "The app name was not package_name!",
        );
    }

    #[test]
    pub fn test_dump() {
        let mut manifest = AbstractManifest::default();
        let debian_control_dump = dump(&manifest);
        assert!(
            !debian_control_dump.is_empty(),
            "The dump from debian::parse was empty!",
        );

    }

    #[test]
    pub fn test_file_path_matches() {
        assert!(file_path_matches("control"));
        assert!(file_path_matches("/path/to/control"));
        assert!(!file_path_matches("/path/to/file.yaml"));
        assert!(!file_path_matches("/path/to/file.json"));
        assert!(!file_path_matches(""));
        assert!(!file_path_matches("/////////////"));
    }
}
