use std::env;

pub fn get_all() -> Vec<crate::projects::project::Project> {
    let core_projects = self::get_core_projects();

    let projects_dir = env::var("PANBUILD_PROJECTS_DIR").unwrap_or(String::from("")).to_string();
    if projects_dir.is_empty() {
        return core_projects;
    }

    // TODO validate the directory!
    return core_projects;
}

pub fn get_core_projects() -> Vec<crate::projects::project::Project> { vec![
    crate::projects::project::Project {
        id: "id".to_string(),
        name: "name".to_string(),
        summary: "name".to_string(),
        description: "name".to_string(),
        homepage: "".to_string(),
        url: "".to_string(),
        versions: vec![],
        maintainers: vec![],
        keywords: vec![],
        dependencies: vec![],
        layer: 0,
    }
]}
