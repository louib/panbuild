pub struct Project {
    id: String,
    name: String,
    urls: Vec<String>,
    available_versions: Vec<String>,
    maintainers: Vec<String>,
    source_urls: Vec<String>,
    dependencies: Vec<Project>,
}

pub struct Version {
    project: Project,
}


pub fn get_all() -> Vec<String> {
    return vec![];
}
