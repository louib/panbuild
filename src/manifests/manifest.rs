use std::error::Error;

// This is the syntax to import sibling modules.
// #[path = "debian.rs"]
// mod debian;
//
// It can also be on the same line!
// #[path = "other_utils/debian.rs"] mod debian;

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
