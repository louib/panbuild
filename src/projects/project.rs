use std::process::Command;

pub enum URLType {
    git,
    hg,
    tarball,
    archive,
}

impl Default for URLType {
    fn default() -> Self { URLType::git }
}

#[derive(Default)]
pub struct Version {
    pub project: Project,
    // Name of the version. Normally follows sem ver.
    pub name: String,
    pub url: String,
    pub url_type: URLType,
    pub tag: String,
    pub branch: String,
}

#[derive(Default)]
pub struct Project {
    pub id: String,
    pub name: String,
    // Basically a short description, or a title.
    pub summary: String,
    pub description: String,
    pub homepage: String,
    pub url: String,
    pub maintainers: Vec<String>,
    pub keywords: Vec<String>,
    pub versions: Vec<Version>,
    pub dependencies: Vec<Version>,
}

pub fn fetch_project(project: Project){
    if project.url.starts_with("https") && project.url.ends_with(".git") {
        let temp_path: &str = "/temp/dir";
        let clone_output = Command::new("git")
            .arg("clone")
            .arg(project.url)
            .arg(temp_path)
            .output()
            .expect("failed to clone git repo.");
        if ! clone_output.status.success() {
            panic!("The clone did not work :(");
        }
    }
}
