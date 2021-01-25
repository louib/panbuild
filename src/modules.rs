pub mod db;
pub mod module;

use std::fs;
use std::path;

use uuid::Uuid;

pub fn get_modules() -> Vec<module::SoftwareModule> {
    db::get_all()
}

pub fn search_modules() {}

pub fn add_module(new_module: &mut module::SoftwareModule) {
    let new_uuid = Uuid::new_v4();
    new_module.id = Some(new_uuid.to_string());
    // FIXME format the names to be valid filenames!
    let mut new_module_path = format!(
        "{}/{}-{}",
        db::MODULES_DIR,
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

pub fn remove_module() {}
