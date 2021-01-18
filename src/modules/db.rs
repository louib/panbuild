use std::env;
use std::fs;

pub fn get_all() -> Vec<crate::modules::module::AbstractModule> {
    let mut core_modules = self::get_core_modules();

    let json_modules_db_path = env::var("PB_MODULES_DB_PATH").unwrap_or(String::from("")).to_string();
    if json_modules_db_path.is_empty() {
        return core_modules;
    }

    let json_modules = match fs::read_to_string(&json_modules_db_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("could not read file {}.", json_modules_db_path);
            return core_modules;
        }
    };
    let mut db_modules: Vec<crate::modules::module::AbstractModule> = serde_json::from_str(&json_modules).unwrap();

    core_modules.append(&mut db_modules);
    core_modules
}

pub fn get_core_modules() -> Vec<crate::modules::module::AbstractModule> {
    vec![]
}
