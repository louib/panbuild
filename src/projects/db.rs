use std::env;
use std::fs;

pub fn get_all() -> Vec<crate::projects::project::Project> {
    let json_projects_db_path = env::var("PB_PROJECTS_DB_PATH").unwrap_or(String::from("")).to_string();
    if json_projects_db_path.is_empty() {
        return vec![];
    }

    let json_projects = match fs::read_to_string(&json_projects_db_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("could not read file {}.", json_projects_db_path);
            return vec![];
        }
    };
    let mut db_projects: Vec<crate::projects::project::Project> = serde_json::from_str(&json_projects).unwrap();

    db_projects
}
