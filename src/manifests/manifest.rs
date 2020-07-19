use std::error::Error;

extern crate yaml_rust;

// This is the syntax to import sibling modules.
// #[path = "debian.rs"]
// mod debian;
//
// It can also be on the same line!
// #[path = "other_utils/debian.rs"] mod debian;

// use yaml_rust::{YamlLoader, YamlEmitter};
use yaml_rust::{YamlLoader};

// Base interface for a build manifest.
trait Manifest {
    fn get_name(&self);
    fn get_version(&self) -> String;

    // Parse a manifest file.
    fn parse(&self);

    fn to_string(&self) -> String;

    // This function detects if a file path could be a potential candidate
    // for this build system.
    fn path_is_manifest_type(&self);
}

trait Package {
    // name of the package. might be unique or not.
    fn get_name(&self) -> String;

    // sem ver
    fn get_version(&self) -> String;

    // URL of the source
    fn get_source(&self) -> String;

    // archive, git, etc...
    fn get_source_type(&self) -> String;
}

// Determines if the filename is a potential manifest
// of any supported build system.
pub fn match_filename(filename: String) -> bool {
    return false;
}

pub fn parse(manifest_content: String, ctx: &ConversionContext) {
    if ctx.source_type == "debian" {
        let lines = manifest_content.split("\n");
        // let mut paragraphs = Vec<Vec<String>>;
        let mut count = 0;
        for line in lines {
            print!("***** {}", line);
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
        return;
    }

    if ctx.source_type == "snap" {
        // let yml_load_result = YamlLoader::load_from_str(&manifest_content);

        // if yml_load_result.is_err() {
            // return;
        // }

        // let manifest_content = yml_load_result.unwrap();
        //
    }

    return;
}
