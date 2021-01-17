pub mod db;
pub mod project;

// Dump the project in the format required by the database.rs file.
pub fn dump_project(project: crate::projects::project::Project) -> String {
    return serde_json::to_string(&project).unwrap();
}

pub fn parse_project(serialized_project: String) -> crate::projects::project::Project {
    let project: crate::projects::project::Project = serde_json::from_str(&serialized_project).unwrap();
    return project;
}

pub fn get_projects() -> Vec<project::Project> {
    return db::get_all();
}

// Get the potential modules that are inferable from the
// projects.
pub fn get_modules() -> Vec<crate::manifests::manifest::AbstractModule> {
    let mut modules = vec![];
    for project in db::get_all() {
        for project_version in &project.versions {
            for artifact_name in &project.artifact_names {
                let mut module = crate::manifests::manifest::AbstractModule::default();
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
