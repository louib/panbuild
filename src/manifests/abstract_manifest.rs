pub struct AbstractManifest {
    package_name: String,
    package_id: String,
    package_version: String,

    modules: Vec<AbstractModule>,
    permissions: Vec<AbstractPermission>,
    executables: Vec<AbstractExecutable>,
}

impl Default for AbstractManifest {
    fn default() -> Self {
        return AbstractManifest {
            package_name: String::from(""),
            package_id: "".to_string(),
            package_version: "".to_string(),

            modules: vec![],
            permissions: vec![],
            executables: vec![],
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

pub struct AbstractExecutable {
    name: String,
    path: String,
    is_desktop: bool,
    icon_path: String,
}

pub struct AbstractPermission {
    name: String,
    description: String,
}
