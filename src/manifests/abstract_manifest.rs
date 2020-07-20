pub enum PackageType {
    dev,
    release,
}

pub const DEFAULT_PACKAGE_TYPE: PackageType = PackageType::dev;

pub enum Architecture {
    amd64,
    i386,
    armhf,
    spark,
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

pub enum NetTool {
    // https://github.com/curl/curl
    curl,
    // http://git.savannah.gnu.org/cgit/wget.git
    wget,
}

pub enum OS {
    bsd,
    mac,
    ios,
    linux,
    android,
    symbian,
    // Add RT Oses??
    // Add misc Oses like calculators and PAs???
}

// Also called distribution.
pub struct OSVersion {
    pub os: OS,
    pub is_distribution: bool,
    // pub name: String,
    // pub codename: String,
}

const jessie: OSVersion = OSVersion {
    os: OS::linux,
    is_distribution: true,
    // name: String::from("jessie"),
    // codename: String::from("stretch"),
};


// TODO Should we allow those systems to be available
// when the generated manifest will be used? We could
// consider optionally downloading those dependencies
// to ensure the version of the build system...
pub enum BuildSystem {
    make,
    cmake,
    autotools,
    meson,
    cargo,
    maven,
    xcode,
    npm,
    // if ever http://git.savannah.gnu.org/cgit/bash.git
    // git@github.com:bminor/bash.git
    bash,
    pip,
    pip3,
    // if ever git@github.com:PowerShell/PowerShell.git
    // powershell,
    manual,
    // if ever git@github.com:apple/swift.git.
    swift,
    native,
    // perl ??
    // ruby ??
    // simple?
    // haskell??
    // LaTeX??
    // mono??
}


impl Default for BuildSystem {
    fn default() -> Self { BuildSystem::make }
}

#[derive(Default)]
pub struct AbstractModule {
    pub name: String,
    pub version: String,
    pub url: String,
    pub url_type: String,
    pub build_system: BuildSystem,
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
