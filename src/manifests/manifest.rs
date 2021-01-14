use std::fs::{self, DirEntry};
use std::io;
use std::path;
use std::process::Output;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ManifestFormat {
    JSON,
    YAML,
    TEXT,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ReleaseType {
    Dev,
    Release,
}
pub const DEFAULT_RELEASE_TYPE: ReleaseType = ReleaseType::Dev;

#[derive(Debug, Serialize, Deserialize)]
pub enum Architecture {
    Amd64,
    I386,
    Armhf,
    Spark,
    Any,
}
pub const DEFAULT_ARCH: Architecture = Architecture::Any;

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub enum NativeManifest {
    Flatpak(crate::manifests::flatpak::FlatpakManifest),
    Debian(crate::manifests::debian::DebianManifest),
    Snapcraft(crate::manifests::snap::SnapcraftManifest),
}

#[derive(Debug, Serialize, Deserialize)]
/// Generic representation of a build manifest.
pub struct AbstractManifest {
    // The path that the manifest was loaded from.
    pub path: String,
    pub format: ManifestFormat,
    pub native_manifest: Option<NativeManifest>,
}
impl Default for AbstractManifest {
    fn default() -> Self {
        AbstractManifest {
            path: "".to_string(),
            format: ManifestFormat::TEXT,
            native_manifest: None,
        }
    }
}
impl AbstractManifest {
    pub fn load_from_file(file_path: String) -> Option<AbstractManifest> {
        let manifest_content = match fs::read_to_string(path::Path::new(&file_path)) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("could not read manifest file {}: {}.", file_path, e);
                return None;
            }
        };
        let mut native_manifest: Option<NativeManifest> = None;
        if let Some(flatpak_manifest) = crate::manifests::flatpak::FlatpakManifest::parse(&manifest_content) {
            native_manifest = Some(NativeManifest::Flatpak(flatpak_manifest));
        } else if let Some(snapcraft_manifest) = crate::manifests::snap::SnapcraftManifest::parse(&manifest_content) {
            native_manifest = Some(NativeManifest::Snapcraft(snapcraft_manifest));
        } else if let Some(debian_manifest) = crate::manifests::debian::DebianManifest::parse(&manifest_content) {
            native_manifest = Some(NativeManifest::Debian(debian_manifest));
        } else {
            return None;
        }

        let mut manifest_format = ManifestFormat::TEXT;
        if file_path.ends_with(".json") {
            manifest_format = ManifestFormat::JSON;
        } else if file_path.ends_with(".yaml") || file_path.ends_with(".yml") {
            manifest_format = ManifestFormat::YAML;
        }

        let manifest = AbstractManifest {
            path: file_path,
            format: manifest_format,
            native_manifest: native_manifest,
        };
        log::debug!("Parsed abstract manifest. Resulting manifest is {:#?}", &manifest);
        Some(manifest)
    }

    pub fn dump(&self) -> Result<String, String> {
        match &self.native_manifest {
            Some(n) => match n {
                NativeManifest::Flatpak(m) => m.dump(&self.format),
                NativeManifest::Snapcraft(m) => m.dump(&self.format),
                _ => Err("Dumping is not supported for this manifest format.".to_string()),
            },
            None => Err("No manifest to dump!".to_string()),
        }
    }

    pub fn get_modules(&self) -> Result<Vec<AbstractModule>, String> {
        match &self.native_manifest {
            Some(n) => match n {
                NativeManifest::Flatpak(m) => Ok(crate::manifests::flatpak::get_modules(&m)),
                _ => Err("Getting the modules is not supported for this manifest format.".to_string()),
            },
            None => Err("No manifest to get the modules from!".to_string()),
        }
    }

    pub fn add_module(&mut self, module: &AbstractModule) -> Result<Vec<AbstractModule>, String> {
        match &mut self.native_manifest {
            Some(n) => match n {
                NativeManifest::Flatpak(m) => crate::manifests::flatpak::add_module(m, module),
                _ => Err("Getting the modules is not supported for this manifest format.".to_string()),
            },
            None => Err("No manifest to get the modules from!".to_string()),
        }
    }

    pub fn run_build(&self) -> Result<String, String> {
        let output = match &self.native_manifest {
            Some(n) => match n {
                NativeManifest::Flatpak(m) => crate::manifests::flatpak::run_build(self),
                _ => return Err("Running a build is not supported for this manifest format.".to_string()),
            },
            None => return Err("No manifest to run the build with!".to_string()),
        };
        match output {
            Ok(o) => Ok(o),
            Err(e) => Err(e.to_string()),
        }
    }
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

// Also called distribution.
pub struct OSVersion {
    pub os: OS,
    pub is_distribution: bool,
    // pub name: String,
    // pub codename: String,
}

const JESSIE: OSVersion = OSVersion {
    os: OS::Linux,
    is_distribution: true,
    // name: String::from("jessie"),
    // codename: String::from("stretch"),
};

#[derive(Debug, Serialize, Deserialize)]
pub enum PackagingSystem {
    Flatpak,
    Snap,
    Debian,
    Arch,
    Homebrew,
    Unknown,
}
pub const DEFAULT_PACKAGING_SYSTEM: PackagingSystem = PackagingSystem::Unknown;
impl Default for PackagingSystem {
    fn default() -> Self {
        DEFAULT_PACKAGING_SYSTEM
    }
}

// TODO Should we allow those systems to be available
// when the generated manifest will be used? We could
// consider optionally downloading those dependencies
// to ensure the version of the build system...
#[derive(Debug, Serialize, Deserialize)]
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
        return crate::manifests::manifest::DEFAULT_BUILD_SYSTEM;
    }

    pub fn get_manifest(self: &BuildSystem) -> AbstractManifest {
        return AbstractManifest::default();
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
/// TODO use the list of all debian packages at
/// https://packages.debian.org/stable/allpackages
/// TODO use the list of all arch packages at
/// https://www.archlinux.org/packages/
pub struct AbstractModule {
    pub module_type: ModuleType,
    pub name: String,
    pub version: String,
    pub url: String,
    pub url_type: SourceType,
    pub build_system: BuildSystem,
    pub packaging_system: PackagingSystem,
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
    pub is_primary: bool,
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
