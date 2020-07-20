pub fn get_all() -> Vec<crate::projects::project::Project> { vec![
    crate::projects::project::Project {
        id: "id".to_string(),
        name: "name".to_string(),
        urls: vec![],
        available_versions: vec![],
        maintainers: vec![],
        source_urls: vec![],
        dependencies: vec![],
    }
]}
