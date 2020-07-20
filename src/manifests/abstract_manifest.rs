pub struct AbstractManifest {
    package_name: String,
    package_id: String,
    package_version: String,

    modules: Vec<AbstractModule>,
}

impl Default for AbstractManifest {
    fn default() -> Self {
        return AbstractManifest {
            package_name: String::from(""),
            package_id: "".to_string(),
            package_version: "".to_string(),

            modules: vec![],
        };
    }
}

pub struct AbstractModule {
    name: String,
    version: String,
    url: String,
    url_type: String,
    build_system: String,
    install_instructions: String,
}
