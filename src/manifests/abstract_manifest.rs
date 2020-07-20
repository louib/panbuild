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
    pub name: String,
    pub version: String,
    pub url: String,
    pub url_type: String,
    pub build_system: String,
    pub install_instructions: String,
    pub install_path: String,
    // The tag associated with the module, if any.
    pub tag: String,
    // The hash of the commit associated with the module, if any.
    pub commit: String,
    // The sha256 checksum of the modules.
    pub sha256: String,
    pub config_options: String,
    // Array of files and directories to cleanup after installing.
    pub cleanup_files: Vec<String>,
}

#[derive(Default)]
pub struct AbstractExecutable {
    pub name: String,
    pub path: String,
    pub is_desktop: bool,
    pub is_daemon: bool,
    // Whether or not this is the primary executable of the bundle.
    pub is_primary: bool,
    pub icon_path: String,
}

#[derive(Default)]
pub struct AbstractPermission {
    pub name: String,
    pub description: String,
}
