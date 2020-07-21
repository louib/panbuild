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
// TODO we should define a list of accepted priorities. In this case,
// for the moment, it's all of them, but it might not be the case if we
// add more cases to the shared enum.


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
const PACKAGE_NAME: &str = "Package-Name";
// Can be "any"
const ARCHITECTURE: &str = "Architecture";
const MULTI_ARCH: &str = "Multi-Arch";
const DEPENDS: &str = "Depends";
// A multi-line string
const DESCRIPTION: &str = "Description";


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

fn is_commented_line(line: &str) -> bool {
    let mut first_char_met: bool = false;
    return line.starts_with(|c: char| {
        if c == ' ' {
            return true;
        }
        if c == '\t' {
            return true;
        }
        if ! first_char_met && c == '#' {
            first_char_met = true;
            return first_char_met;
        }
        if first_char_met {
            return true;
        }
        return false;
    });
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
        if is_commented_line(line) {
            continue;
        }
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            // FIXME we should return a Result<> instead of exiting here or
            // returning a default value.
            eprintln!("Invalid debian control file line {}", line);
            return response;
        }
        let field_name = parts[0];
        let field_value = parts[1];

        println!("field_name: {}", field_name.to_string());

        if field_name == SOURCE {
            response.package_name = field_value.to_string();
        }
    }

    // TODO validate that there is more than 1 paragraph?
    for paragraph_index in 1..paragraphs.len() {
        let paragraph = &paragraphs[paragraph_index];
        for line in paragraph.split('\n') {
            if line.starts_with(" ") {
                // Obviously a mistake
                continue;
            }
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() != 2 {
                // FIXME we should return a Result<> instead of exiting here or
                // returning a default value.
                eprintln!("Invalid debian control file line {}", line);
                return response;
            }
            let field_name = parts[0];
            let field_value = parts[1];

        }

        let mut package = AbstractModule::default();

        package.name = "name".to_string();
        // architecture =
        // multi_arch =
        // depends =
        // description =
        response.modules.push(package);
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
    return false;
}

pub fn file_content_matches(content: &str) -> bool {
    return false;
}

const debian_control_example: &str = r###"
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
mod debian_tests {
    use super::*;

    #[test]
    pub fn test_parse() {
        let mut manifest = parse(&debian_control_example);
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
}
