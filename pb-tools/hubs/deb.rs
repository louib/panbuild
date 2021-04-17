use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DebianPackagesHub {}
impl DebianPackagesHub {
    pub fn get_modules_from_debian_repository(repo_name: &str, repo_sources_url: &str) -> Vec<panbuild::modules::SoftwareModule> {
        vec![]
    }
}
