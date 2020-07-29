pub mod project;
pub mod db;

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
