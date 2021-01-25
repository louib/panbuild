use std::env;
use std::fs;
use std::path;

use uuid::Uuid;

pub const DEFAULT_DB_PATH: &str = "~/.panbuild-db";
pub const MODULES_DB_SUBDIR: &str = "/modules";
pub const PROJECTS_DB_SUBDIR: &str = "/projects";

pub struct Database {
    pub projects: Vec<crate::projects::project::Project>,
    pub modules: Vec<crate::modules::SoftwareModule>,
}
impl Database {
    pub fn get_database() -> Database {
        // FIXME error handle the init.
        Database {
            projects: Database::get_all_projects(),
            modules: Database::get_all_modules(),
        }
    }

    pub fn get_db_path() -> String {
        match env::var("PB_DB_PATH") {
            Ok(path) => return path.to_string(),
            Err(e) => return "~/.panbuild".to_string(),
        }
    }

    pub fn get_modules_db_path() -> String {
        Database::get_db_path() + MODULES_DB_SUBDIR
    }

    pub fn get_projects_db_path() -> String {
        Database::get_db_path() + PROJECTS_DB_SUBDIR
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

    pub fn get_all_modules() -> Vec<crate::modules::SoftwareModule> {
        let modules_path = Database::get_modules_db_path();
        let modules_path = path::Path::new(&modules_path);
        let all_modules_paths = match crate::utils::get_all_paths(modules_path) {
            Ok(paths) => paths,
            Err(e) => {
                return vec![];
            }
        };
        let mut modules: Vec<crate::modules::SoftwareModule> = vec![];
        for module_path in all_modules_paths.iter() {
            if !module_path.is_file() {
                log::debug!("{} is not a file.", &module_path.to_str().unwrap());
                continue;
            }
            let module_content = match fs::read_to_string(module_path) {
                Ok(content) => content,
                Err(e) => {
                    log::debug!("Could not read module file {}: {}.", &module_path.to_str().unwrap(), e);
                    continue;
                }
            };
            let module = match serde_yaml::from_str(&module_content) {
                Ok(m) => m,
                Err(e) => {
                    log::debug!("Could not parse module file at {}: {}.", &module_path.to_str().unwrap(), e);
                    continue;
                }
            };
            modules.push(module);
        }
        modules
    }

    pub fn search_modules() {}
    pub fn remove_module() {}

    pub fn add_module(new_module: &mut crate::modules::SoftwareModule) {
        let new_uuid = Uuid::new_v4();
        new_module.id = Some(new_uuid.to_string());
        let modules_path = Database::get_modules_db_path();
        let mut new_module_path = format!(
            "{}/{}-{}",
            modules_path,
            crate::utils::normalize_name(&new_module.name),
            new_module.id.as_ref().unwrap()
        );
        log::info!("Adding module at {}", new_module_path);
        let mut new_module_fs_path = path::Path::new(&new_module_path);
        if new_module_fs_path.exists() {
            panic!("Path {} already exists. This should not happen!!", new_module_path);
        }
        match fs::write(new_module_fs_path, serde_yaml::to_string(&new_module).unwrap()) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("could not write new module at {}.", new_module_path.to_string());
            }
        };
    }
}
