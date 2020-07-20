pub mod project;
pub mod db;

// Dump the project in the format required by the database.rs file.
pub fn dump_project() -> String {
    return "".to_string();
}

pub fn get_projects() -> Vec<project::Project> {
    return db::get_all();
}
