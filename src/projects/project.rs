use serde::{Serialize, Deserialize};

use std::process::Command;

pub const PROJECTS_DIR: &str = "~/.panbuild/projects/";

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Default)]
pub struct ProjectVersion {
    pub project_id: String,
    // Name of the version. Normally follows sem ver.
    pub name: String,
    pub url: String,
    pub url_type: crate::manifests::manifest::SourceType,
    pub tag: String,
    pub branch: String,
    pub sha256sum: String,
    pub dependencies: Vec<Dependancy>,
}

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Default)]
pub struct Project {
    pub id: String,
    pub name: String,
    // Basically a short description, or a title.
    pub summary: String,
    pub description: String,
    pub web_urls: Vec<String>,
    pub cvs_urls: Vec<String>,
    pub maintainers: Vec<String>,
    pub keywords: Vec<String>,
    // Whether the project is part of the internal projects db.
    pub is_core: bool,

    // Layer of the project. This means how central the project is to the
    // open source ecosystem in general. 0 being the most central layer
    // (firmwares, bootloaders, kernels, compilers, core utilities).
    //
    // Beyond maybe 0 and 1, the exact layer should not be calculated manually,
    // but rather a spread factor and a max layer should be configured.
    pub layer: i32,
}
impl Project {
    // Serializes the project to a native Rust struct creation.
    // This is used to include projects to the internal db.
    fn to_rust(self: &Self) -> String {
        return String::from(r#####"
        crate::projects::project::Project {
            id: "{}".to_string(),
            name: "{}".to_string(),
            summary: "{}".to_string(),
            description: "{}".to_string(),
            web_urls: vec![],
            layer: 7,
            is_core: false,
        }
        "#####);
    }
}

// See https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md
// and https://doc.rust-lang.org/cargo/reference/manifest.html#the-version-field
#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Default)]
pub struct SemanticVersion {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
}

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Default)]
pub struct Dependancy {
    pub min_version: SemanticVersion,
    pub max_version: SemanticVersion,
    pub project_id: String,
}
