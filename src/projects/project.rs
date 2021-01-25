use serde::{Deserialize, Serialize};

use std::process::Command;

#[derive(Serialize, Deserialize, Default)]
pub struct Project {
    // Unique project id, if available
    pub id: String,
    pub name: String,
    // Basically a short description, or a title.
    pub summary: String,
    pub description: String,
    pub web_urls: Vec<String>,
    pub vcs_urls: Vec<String>,
    // Name of the artifacts that this project produces. Can be binaries, libraries or assets.
    pub artifact_names: Vec<String>,
    pub maintainers: Vec<String>,
    // Thos
    pub versions: Vec<String>,
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
        let mut response: String = String::from("crate::projects::project::Project {\n");
        response.push_str(&format!("    id: \"{}\".to_string(),", self.id).to_string());
        response.push_str(&format!("    name: \"{}\".to_string(),", self.name).to_string());
        response.push_str(&format!("    summary: \"{}\".to_string(),", self.summary).to_string());
        response.push_str(&format!("    description: \"{}\".to_string(),", self.description).to_string());
        response.push_str("}");
        response
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct ProjectVersion {
    pub project_id: String,
    // Name of the version. Normally follows sem ver.
    pub name: String,
    pub url: String,
    pub url_type: crate::modules::SourceType,
    pub tag: String,
    pub branch: String,
    pub sha256sum: String,
    pub dependencies: Vec<Dependancy>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Dependancy {
    pub min_version: crate::version::SemanticVersion,
    pub max_version: crate::version::SemanticVersion,
    pub project_id: String,
}
