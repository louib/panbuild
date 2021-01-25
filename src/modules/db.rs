use std::env;
use std::fs;

pub const MODULES_DIR: &str = "~/.panbuild/modules/";

pub fn get_all() -> Vec<crate::modules::module::SoftwareModule> {
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
