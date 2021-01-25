use std::env;
use std::fs;

pub const MODULES_DIR: &str = "~/.panbuild/modules/";

pub struct Database {
    pub projects: Vec<crate::projects::project::Project>,
    pub modules: Vec<crate::modules::module::SoftwareModule>,
}
impl Database {
    pub fn get_loaded_database() -> Option<Vec<Database>> {
        None
    }

    pub fn get_all_projects() -> Vec<crate::projects::project::Project> {
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

    pub fn get_all_modules() -> Vec<crate::modules::module::SoftwareModule> {
        let json_modules_db_path = env::var("PB_MODULES_DB_PATH").unwrap_or(String::from("")).to_string();
        if json_modules_db_path.is_empty() {
            return vec![];
        }

        let json_modules = match fs::read_to_string(&json_modules_db_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("could not read file {}.", json_modules_db_path);
                return vec![];
            }
        };
        let mut db_modules: Vec<crate::modules::module::SoftwareModule> = serde_json::from_str(&json_modules).unwrap();

        db_modules
    }

    pub fn search_modules() {}
    pub fn remove_module() {}
}
