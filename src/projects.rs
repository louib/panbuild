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
