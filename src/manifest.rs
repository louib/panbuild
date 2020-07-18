use std::error::Error;

extern crate yaml_rust;

pub struct ConversionContext {
    pub source_type: String,
    pub destination_type: String,
    pub content: String,
}

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

pub fn get_type(manifest_path: String, ret: &str) -> &str {
    // TODO match agains regexes.

    return ret;
}

pub fn parse(manifest_content: String, ctx: &ConversionContext) {
    let yml_load_result = YamlLoader::load_from_str(&manifest_content);

    if yml_load_result.is_err() {
        return;
    }

    let manifest_content = yml_load_result.unwrap();
    return;
}

pub fn dump(manifest: &Manifest, destination_type: String) -> String {
    return String::new();
}
