#[derive(Default)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub urls: Vec<String>,
    pub available_versions: Vec<String>,
    pub maintainers: Vec<String>,
    pub source_urls: Vec<String>,
    pub dependencies: Vec<String>,
}

#[derive(Default)]
pub struct Version {
    pub project: Project,
    pub semver: String,
}
