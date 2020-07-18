extern crate yaml_rust;

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

pub fn get_manifest(manifest_content: String, manifest_type: String) {
    let manifest = YamlLoader::load_from_str(&manifest_content);

    return;
}

pub fn convert() {}
