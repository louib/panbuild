pub mod project;
pub mod db;

// Dump the project in the format required by the database.rs file.
pub fn dump_project(project: crate::projects::project::Project) -> String {
    let mut response: String = String::from("    crate::projects::project::Project {\n");
    response.push_str("        id: \"{id}\".to_string(),");
    response.push_str("        name: \"{name}\".to_string(),");
    response.push_str("        homepage: \"{homepage}\".to_string(),");
    response.push_str("        urls: \"{homepage}\".to_string(),");
    response.push_str("        maintainers: vec![],");
    response.push_str("        dependencies: vec![],");
    if project.versions.len() != 0 {
        response.push_str("        versions: vec![\n");
        for url in project.versions {
            // response.push_str(&format!("            \"{}\",\n", url));
        }
        response.push_str("],\n");
    } else {
        response.push_str("        versions: vec![],");
    }
    response.push_str("        name: \"{name}\".to_string(),");

    return response;
}

pub fn get_projects() -> Vec<project::Project> {
    return db::get_all();
}
