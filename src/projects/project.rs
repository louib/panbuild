pub enum URLType {
    git,
    hg,
    tarball,
    archive,
}

impl Default for URLType {
    fn default() -> Self { URLType::git }
}

#[derive(Default)]
pub struct Version {
    pub project: Project,
    // Name of the version. Normally follows sem ver.
    pub name: String,
    pub url: String,
    pub url_type: URLType,
    pub tag: String,
    pub branch: String,
}

#[derive(Default)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub homepage: String,
    pub url: String,
    pub maintainers: Vec<String>,
    pub versions: Vec<Version>,
    pub dependencies: Vec<Version>,
}
