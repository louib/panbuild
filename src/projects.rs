use serde::{Deserialize, Serialize};

use std::process::Command;

pub const CORE_PROJECTS: [&'static str; 20] = [
    "https://git.savannah.gnu.org/cgit/bash.git",
    "https://git.savannah.gnu.org/cgit/make.git",
    "https://git.savannah.gnu.org/cgit/diffutils.git",
    "https://git.savannah.gnu.org/cgit/findutils.git",
    "https://git.savannah.gnu.org/cgit/gzip.git",
    "https://git.savannah.gnu.org/git/grep.git",
    "https://git.savannah.gnu.org/cgit/tar.git",
    "https://git.savannah.gnu.org/git/libtool.git",
    "https://git.lysator.liu.se/lsh/lsh.git",
    "https://git.savannah.gnu.org/cgit/gawk.git",
    "https://github.com/gwsw/less.git",
    "https://github.com/openbsd/src.git",
    "https://gcc.gnu.org/git/gcc.git",
    "https://git.sv.gnu.org/cgit/coreutils.git",
    "https://sourceware.org/git/binutils-gdb.git",
    "https://sourceware.org/git/glibc.git",
    "https://gitlab.gnome.org/GNOME/gtk.git",
    "https://gitlab.gnome.org/GNOME/glib.git",
    "https://dev.gnupg.org/source/gnupg.git",
    "https://gitlab.com/gnutls/gnutls.git",
];

#[derive(Serialize, Deserialize, Default)]
pub struct SoftwareProject {
    // Project ids are based on the reverse DNS notation, and
    // are either derived from build manifests found in the project
    // using the same reverse DNS notation, or from the git urls
    // associated to the project.
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
    pub default_branch: String,
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
impl SoftwareProject {
    // Serializes the project to a native Rust struct creation.
    // This is used to include projects to the internal db.
    fn to_rust(self: &Self) -> String {
        let mut response: String = String::from("crate::projects::SoftwareProject {\n");
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
    pub dependencies: Vec<Dependency>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Dependency {
    pub min_version: crate::version::SemanticVersion,
    pub max_version: crate::version::SemanticVersion,
    pub project_id: String,
}

// Get the potential modules that are inferable from the
// projects.
pub fn get_modules() -> Vec<crate::modules::SoftwareModule> {
    let mut modules = vec![];
    for project in crate::db::Database::get_all_projects() {
        for project_version in &project.versions {
            for artifact_name in &project.artifact_names {
                let mut module = crate::modules::SoftwareModule::default();
                module.name = artifact_name.to_string();
                module.version = project_version.to_string();
                module.tag = project_version.to_string();
                if project.vcs_urls.len() != 0 {
                    module.url = project.vcs_urls[0].to_string();
                }
                modules.push(module);
                // println!("Project {} could install {} version {}.", project.name, artifact_name, project_version);
            }
        }
    }
    modules
}

pub fn get_project_tag_names() -> Vec<String> {
    // call tag_names(&self, pattern: Option<&str>) -> Result<StringArray, Error>
    // on the repository.
    // https://docs.rs/git2/0.13.8/git2/struct.Repository.html#method.tag_names
    return vec![];
}

pub fn get_project_commit_signature() -> String {
    // Here we want to asses a project signature by traversing the Revision graph.
    // https://docs.rs/git2/0.13.8/git2/struct.Revwalk.html
    return String::from("");
}
