use std::process::Command;
use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

pub const PROJECTS_DIR: &str = "~/.panbuild/projects/";

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
        let clone_output = Command::new("mkdir")
            .arg("-p")
            .arg(PROJECTS_DIR)
            .output()
            .expect(&format!("failed to create projects directory at {}!", PROJECTS_DIR).to_string());
        if ! clone_output.status.success() {
            panic!("The clone did not work :(");
        }

        let clone_output = Command::new("git")
            .arg("clone")
            .arg(&project.url)
            .arg(PROJECTS_DIR)
            .output()
            .expect(&format!("failed to clone git repository at {}!", &project.url).to_string());
        if ! clone_output.status.success() {
            panic!("The clone did not work :(");
        }
    }
}

// one possible implementation of walking a directory only visiting files
// Taken from https://doc.rust-lang.org/std/fs/fn.read_dir.html
fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}
