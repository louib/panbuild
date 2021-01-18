pub mod db;
pub mod module;

use uuid::Uuid;

pub fn get_modules() -> Vec<module::AbstractModule> {
    db::get_all()
}

pub fn search_modules() {}

pub fn add_module(new_module: &mut module::AbstractModule) {
    let new_uuid = Uuid::new_v4();
    new_module.id = Some(new_uuid.to_string());
    // FIXME format the names to be valid filenames!
    let filename = format!("{}-{}", new_module.name, new_module.id.as_ref().unwrap());
}

pub fn remove_module() {}
