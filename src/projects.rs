#[derive(Default)]
pub struct Project {
    id: String,
    name: String,
    urls: Vec<String>,
    available_versions: Vec<String>,
    maintainers: Vec<String>,
    source_urls: Vec<String>,
    dependencies: Vec<Project>,
}

#[derive(Default)]
pub struct Version {
    project: Project,
    semver: String,
}

pub fn get_all() -> Vec<String> {
    return vec![];
}
