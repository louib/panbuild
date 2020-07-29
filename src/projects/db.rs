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
        id: "flatpak-builder".to_string(),
        name: "flatpak-builder".to_string(),
        summary: "Tool to build flatpaks from source".to_string(),
        description: "
            Flatpak-builder is a tool for building flatpaks from sources.
            See http://flatpak.org/ for more information.
        ".to_string(),
        homepage: "http://flatpak.org/".to_string(),
        url: "".to_string(),
        versions: vec![],
        maintainers: vec![],
        keywords: vec![],
        dependencies: vec![],
        layer: 0,
        is_core: true,
    },
]}
