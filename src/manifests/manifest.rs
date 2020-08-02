use serde::{Serialize, Deserialize};

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub enum PackageType {
    App,
    Lib,
    Driver,
    Daemon,
    Kernel,
    Plugin,
    Runtime,
    Emulator,
    Compiler,
    Bootloader,
    Firmware,
    Media,
}
pub const DEFAULT_PACKAGE_TYPE: PackageType = PackageType::App;

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub enum ReleaseType {
    Dev,
    Release,
}
pub const DEFAULT_RELEASE_TYPE: ReleaseType = ReleaseType::Dev;

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub enum Architecture {
    Amd64,
    I386,
    Armhf,
    Spark,
    Any,
}
pub const DEFAULT_ARCH: Architecture = Architecture::Any;

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
/// Software license used for a package.
/// See https://spdx.org/licenses/ For the complete list of commonly found
/// free and open source licenses.
pub enum License {
    Gpl2,
    Gpl3,
    Mit,
    Bsd2,
    Bsd3,
    Proprietary,
    Unknown,
}
pub const DEFAULT_LICENSE: License = License::Gpl2;


#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
/// Generic representation of a build manifest.
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

    // The main module that this manifest is installing.
    pub main_module: AbstractModule,
    // The modules that the module being built requires.
    pub depends_on: Vec<AbstractModule>,

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

            main_module: AbstractModule::default(),
            depends_on: vec![],
            permissions: vec![],
            executables: vec![],
        }
    }
}
// We implement the Iterator Trait to offer a convenient
// way to travers the package tree recursively.
impl Iterator for AbstractManifest {
    type Item = AbstractModule;

    fn next(&mut self) -> Option<AbstractModule> {
        return None;
        //Some(self.curr);
    }
}
impl AbstractManifest {
    fn dump(&self) -> String {
        return serde_json::to_string(&self).unwrap_or(String::from(""));
    }
    fn parse(content: &str) -> AbstractManifest {
        return AbstractManifest::default();
    }
}

pub enum NetTool {
    // https://github.com/curl/curl
    Curl,
    // http://git.savannah.gnu.org/cgit/wget.git
    Wget,
}

pub enum OS {
    Bsd,
    Mac,
    Ios,
    Linux,
    Android,
    Symbian,
    // Add RT Oses??
    // Add misc Oses like calculators and PAs???
}

// See https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md
// and https://doc.rust-lang.org/cargo/reference/manifest.html#the-version-field
pub struct SemanticVersion {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
}

// Also called distribution.
pub struct OSVersion {
    pub os: OS,
    pub is_distribution: bool,
    // pub name: String,
    // pub codename: String,
}

const jessie: OSVersion = OSVersion {
    os: OS::Linux,
    is_distribution: true,
    // name: String::from("jessie"),
    // codename: String::from("stretch"),
};

// TODO Should we allow those systems to be available
// when the generated manifest will be used? We could
// consider optionally downloading those dependencies
// to ensure the version of the build system...
#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub enum BuildSystem {
    Make,
    Cmake,
    Autotools,
    Meson,
    Cargo,
    Maven,
    Xcode,
    Npm,
    // if ever http://git.savannah.gnu.org/cgit/bash.git
    // git@github.com:bminor/bash.git
    Bash,
    Pip2,
    Pip3,
    // if ever git@github.com:PowerShell/PowerShell.git
    // powershell,
    Manual,
    // if ever git@github.com:apple/swift.git.
    Swift,
    Apt,
    // perl ??
    // ruby ??
    // simple?
    // haskell??
    // LaTeX??
    // mono??
    Unknown,
}

pub const DEFAULT_BUILD_SYSTEM: BuildSystem = BuildSystem::Unknown;

impl Default for BuildSystem {
    fn default() -> Self { DEFAULT_BUILD_SYSTEM }
}

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub enum SourceType {
    Bzr,
    Deb,
    Git,
    Local,
    Mercurial,
    Rpm,
    Subversion,
    Svn,
    Tar,
    Zip,
    // 7z
    Sevenzip,
    Unknown,
}

pub const DEFAULT_SOURCE_TYPE: SourceType = SourceType::Unknown;

impl Default for SourceType {
    fn default() -> Self { DEFAULT_SOURCE_TYPE }
}

#[derive(Default)]
#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
/// Generic representation of a software module.
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
    pub depends_on: Vec<AbstractModule>,
}

#[derive(Default)]
#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
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
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct AbstractPermission {
    pub name: String,
    pub description: String,
    pub api_type: APIType,
}

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub enum APIType {
    Dbus,
    Files,
    Socket,
    Camera,
    Mic,
    Gps,
    Unknown,
}

pub const DEFAULT_API_TYPE: APIType = APIType::Unknown;

impl Default for APIType {
    fn default() -> Self { DEFAULT_API_TYPE }
}

// Currently the documentation comes from the Debian control file documentation.
pub enum Priority {
    // Packages which are necessary for the proper functioning of the system (usually,
    // this means that dpkg functionality depends on these packages).
    // Removing a required package may cause your system to become totally broken and you
    // may not even be able to use dpkg to put things back,
    // so only do so if you know what you are doing.
    //
    // Systems with only the required packages installed have at least enough functionality for
    // the sysadmin to boot the system and install more software.
    Required,

    // Important programs, including those which one would expect to find on any Unix-like system.
    // If the expectation is that an experienced Unix person who found it missing would
    // say “What on earth is going on, where is foo?”,
    // it must be an important package. 6 Other packages without which the system will not run
    // well or be usable must also have priority important.
    // This does not include Emacs, the X Window System, TeX or any other large applications.
    // The important packages are just a bare minimum of commonly-expected and necessary tools.
    Important,

    // These packages provide a reasonably small but not too limited character-mode system.
    // This is what will be installed by default if the user doesn’t select anything else.
    // It doesn’t include many large applications.
    //
    // No two packages that both have a priority of standard or higher may conflict with each other.
    Standard,

    // This is the default priority for the majority of the archive.
    // Unless a package should be installed by default on standard Debian systems,
    // it should have a priority of optional. Packages with a priority of optional
    // may conflict with each other.
    Optional,

    // This priority is deprecated. Use the optional priority instead.
    // This priority should be treated as equivalent to optional.
    //
    // The extra priority was previously used for packages that conflicted with other packages and packages
    // that were only likely to be useful to people with specialized requirements. However, this distinction
    // was somewhat arbitrary, not consistently followed, and not useful enough to warrant
    // the maintenance effort.
    Extra,
}

const DEFAULT_PRIORITY: Priority = Priority::Optional;
