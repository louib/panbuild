#[derive(Debug)]
pub enum PackageType {
    app,
    lib,
    driver,
    daemon,
    kernel,
    plugin,
    runtime,
}
pub const DEFAULT_PACKAGE_TYPE: PackageType = PackageType::app;

#[derive(Debug)]
pub enum ReleaseType {
    dev,
    release,
}
pub const DEFAULT_RELEASE_TYPE: ReleaseType = ReleaseType::dev;

#[derive(Debug)]
pub enum Architecture {
    amd64,
    i386,
    armhf,
    spark,
    any,
}
pub const DEFAULT_ARCH: Architecture = Architecture::any;

#[derive(Debug)]
pub enum License {
    gpl2,
    gpl3,
    mit,
    bsd2,
    bsd3,
    proprietary,
    other,
}
pub const DEFAULT_LICENSE: License = License::gpl2;


#[derive(Debug)]
pub struct AbstractManifest {
    pub package_name: String,
    pub package_id: String,
    pub package_version: String,
    pub short_description: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub package_type: PackageType,
    pub release_type: ReleaseType,
    pub architecture: Architecture,
    pub license: License,

    pub modules: Vec<AbstractModule>,
    pub permissions: Vec<AbstractPermission>,
    pub executables: Vec<AbstractExecutable>,
}

impl Default for AbstractManifest {
    fn default() -> Self {
        AbstractManifest {
            package_name: String::from(""),
            package_id: "".to_string(),
            package_version: "".to_string(),

            short_description: "".to_string(),
            description: "".to_string(),
            keywords: vec![],
            package_type: DEFAULT_PACKAGE_TYPE,
            release_type: DEFAULT_RELEASE_TYPE,
            architecture: DEFAULT_ARCH,
            license: DEFAULT_LICENSE,

            modules: vec![],
            permissions: vec![],
            executables: vec![],
        }
    }
}
impl AbstractManifest {
    fn dump(&self) -> String {
        return String::from("");
    }
    fn parse(content: &str) -> AbstractManifest {
        return AbstractManifest::default();
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
#[derive(Debug)]
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
    unknown,
}

pub const DEFAULT_BUILD_SYSTEM: BuildSystem = BuildSystem::unknown;

impl Default for BuildSystem {
    fn default() -> Self { DEFAULT_BUILD_SYSTEM }
}

#[derive(Debug)]
pub enum SourceType {
    bzr,
    deb,
    git,
    hg,
    local,
    mercurial,
    rpm,
    subversion,
    svn,
    tar,
    zip,
    // 7z
    sevenzip,
    unknown,
}

pub const DEFAULT_SOURCE_TYPE: SourceType = SourceType::unknown;

impl Default for SourceType {
    fn default() -> Self { DEFAULT_SOURCE_TYPE }
}

#[derive(Default)]
#[derive(Debug)]
pub struct AbstractModule {
    pub name: String,
    pub version: String,
    pub url: String,
    pub url_type: SourceType,
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
#[derive(Debug)]
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
#[derive(Debug)]
pub struct AbstractPermission {
    pub name: String,
    pub description: String,
    pub api_type: APIType,
}

#[derive(Debug)]
pub enum APIType {
    dbus,
    fs,
    socket,
    camera,
    mic,
    gps,
    unknown,
}

impl Default for APIType {
    fn default() -> Self { APIType::unknown }
}

// Currently the documentation comes from the Debian control file documentation.
pub enum Priority {
    // Packages which are necessary for the proper functioning of the system (usually, this means that dpkg functionality depends on these packages).
    // Removing a required package may cause your system to become totally broken and you may not even be able to use dpkg to put things back,
    // so only do so if you know what you are doing.
    //
    // Systems with only the required packages installed have at least enough functionality for the sysadmin to boot the system and install more software.
    required,

    // Important programs, including those which one would expect to find on any Unix-like system.
    // If the expectation is that an experienced Unix person who found it missing would say “What on earth is going on, where is foo?”,
    // it must be an important package. 6 Other packages without which the system will not run well or be usable must also have priority important.
    // This does not include Emacs, the X Window System, TeX or any other large applications.
    // The important packages are just a bare minimum of commonly-expected and necessary tools.
    important,

    // These packages provide a reasonably small but not too limited character-mode system.
    // This is what will be installed by default if the user doesn’t select anything else.
    // It doesn’t include many large applications.
    //
    // No two packages that both have a priority of standard or higher may conflict with each other.
    standard,

    // This is the default priority for the majority of the archive.
    // Unless a package should be installed by default on standard Debian systems,
    // it should have a priority of optional. Packages with a priority of optional may conflict with each other.
    optional,

    // This priority is deprecated. Use the optional priority instead.
    // This priority should be treated as equivalent to optional.
    //
    // The extra priority was previously used for packages that conflicted with other packages and packages
    // that were only likely to be useful to people with specialized requirements. However, this distinction
    // was somewhat arbitrary, not consistently followed, and not useful enough to warrant the maintenance effort.
    extra,
}
