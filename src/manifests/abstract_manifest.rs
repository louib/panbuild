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

#[derive(Default)]
pub struct AbstractModule {
    name: String,
    version: String,
    url: String,
    url_type: String,
    build_system: String,
    install_instructions: String,
    // The tag associated with the module, if any.
    tag: String,
    // The hash of the commit associated with the module, if any.
    commit: String,
    // The sha256 checksum of the modules.
    sha256: Option<String>,
    config_options: Option<String>,
    // Array of files and directories to cleanup after installing.
    cleanup_files: Vec<String>,

    misc_config: String,
}

pub struct AbstractExecutable {
    name: String,
    path: String,
    is_desktop: bool,
    is_daemon: bool,
    icon_path: String,
}

pub struct AbstractPermission {
    name: String,
    description: String,
}
