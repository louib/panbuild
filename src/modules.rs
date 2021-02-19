use serde::{Deserialize, Serialize};

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

// TODO Should we allow those systems to be available
// when the generated manifest will be used? We could
// consider optionally downloading those dependencies
// to ensure the version of the build system...
#[derive(Debug, Serialize, Deserialize)]
pub enum BuildSystem {
    Make,
    Cmake,
    Qmake,
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
    Gem,
    // simple?
    // haskell??
    // LaTeX??
    // mono??
    Unknown,
}
impl BuildSystem {
    pub fn get_build_system(path: &str) -> BuildSystem {
        if path.ends_with("meson_options.txt") {
            return BuildSystem::Meson;
        }
        if path.ends_with("control") {
            return BuildSystem::Apt;
        }
        if path.ends_with("package.json") {
            return BuildSystem::Npm;
        }
        if path.ends_with("Gemfile") {
            return BuildSystem::Gem;
        }
        if path.ends_with("requirements.txt") {
            // We could also default to pip2...
            return BuildSystem::Pip3;
        }
        if path.ends_with(".spec") {
            // return crate::manifests::manifest::BuildSystem::Fedora;
        }
        if path.ends_with("Makefile") {
            return BuildSystem::Make;
        }
        return DEFAULT_BUILD_SYSTEM;
    }
}

pub const DEFAULT_BUILD_SYSTEM: BuildSystem = BuildSystem::Unknown;

impl Default for BuildSystem {
    fn default() -> Self {
        DEFAULT_BUILD_SYSTEM
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
    Tarball,
    Zip,
    // 7z
    Sevenzip,
    Unknown,
}

pub const DEFAULT_SOURCE_TYPE: SourceType = SourceType::Unknown;

impl Default for SourceType {
    fn default() -> Self {
        DEFAULT_SOURCE_TYPE
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ModuleType {
    CLIApp,
    GUIApp,
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
    Unknown,
}
pub const DEFAULT_MODULE_TYPE: ModuleType = ModuleType::Lib;
impl Default for ModuleType {
    fn default() -> Self {
        DEFAULT_MODULE_TYPE
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
/// Generic representation of a software module.
pub struct SoftwareModule {
    pub name: String,
    pub project_id: Option<String>,

    // The version of the current module.
    pub version: String,
    // The tag associated with the module, if any.
    pub tag: String,
    // The hash of the commit associated with the module, if any.
    pub commit: String,

    pub module_type: ModuleType,

    pub download_urls: Vec<String>,
    pub url: String,
    pub build_system: BuildSystem,
    pub archive_checksum: String,
    pub source_checksum: String,
    // When we have reproducible builds.
    pub executable_checksum: String,

    // Fields mostly taken from the Flatpak manifest.
    // Array of files and directories to cleanup after installing.
    pub config_options: Vec<String>,
    pub build_commands: Vec<String>,
    pub install_instructions: String,
    pub install_path: String,
}
impl SoftwareModule {
    pub fn get_identifier(&self) -> &str {
        if self.version.len() != 0 {
            return self.version.as_str();
        }
        if self.tag.len() != 0 {
            return self.tag.as_str();
        }
        if self.commit.len() != 0 {
            return self.commit.as_str();
        }
        return "";

    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AbstractExecutable {
    pub name: String,
    pub path: String,
    pub is_desktop: bool,
    pub is_daemon: bool,
    // Whether or not this is the primary executable of the bundle.
    pub is_primary: bool,
    pub icon_path: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AbstractPermission {
    pub name: String,
    pub description: String,
    pub api_type: APIType,
}

#[derive(Debug, Serialize, Deserialize)]
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
    fn default() -> Self {
        DEFAULT_API_TYPE
    }
}
