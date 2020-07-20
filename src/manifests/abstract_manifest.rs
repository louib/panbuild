pub enum PackageType {
    dev,
    release,
}

pub const DEFAULT_PACKAGE_TYPE: PackageType = PackageType::dev;

pub enum Architecture {
    amd64,
    armhf,
    any,
}

pub const DEFAULT_ARCH: Architecture = Architecture::any;

pub struct AbstractManifest {
    pub package_name: String,
    pub package_id: String,
    pub package_version: String,
    pub short_description: String,
    pub description: String,
    pub package_type: PackageType,
    pub architecture: Architecture,

    pub modules: Vec<AbstractModule>,
    pub permissions: Vec<AbstractPermission>,
    pub executables: Vec<AbstractExecutable>,
}

impl Default for AbstractManifest {
    fn default() -> Self {
        return AbstractManifest {
            package_name: String::from(""),
            package_id: "".to_string(),
            package_version: "".to_string(),

            short_description: "".to_string(),
            description: "".to_string(),
            package_type: DEFAULT_PACKAGE_TYPE,
            architecture: DEFAULT_ARCH,

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
    // Whether or not this is the primary executable of the bundle.
    is_primary: bool,
    icon_path: String,
}

pub struct AbstractPermission {
    name: String,
    description: String,
}
