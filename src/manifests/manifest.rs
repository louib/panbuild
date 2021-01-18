use std::fs::{self, DirEntry};
use std::io;
use std::path;
use std::process::Output;

use serde::{Deserialize, Serialize};

use crate::modules::module::AbstractModule;

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
impl License {
    fn parse(license: String) -> License {
        if license.contains("copyright") {
            return License::Proprietary;
        }
        return License::Unknown;
    }
}
pub const DEFAULT_LICENSE: License = License::Gpl2;

#[derive(Debug, Serialize, Deserialize)]
pub enum NativeManifest {
    Flatpak(crate::manifests::flatpak::FlatpakManifest),
    Debian(crate::manifests::debian::DebianManifest),
    Snapcraft(crate::manifests::snap::SnapcraftManifest),
    Javascript(crate::manifests::javascript::JavascriptPackageManifest),
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
    pub fn get_type(&self) -> Option<&str> {
        match &self.native_manifest {
            Some(m) => match m {
                NativeManifest::Flatpak(m) => return Some(m.get_type()),
                NativeManifest::Snapcraft(m) => return Some(m.get_type()),
                NativeManifest::Javascript(m) => return Some(m.get_type()),
                NativeManifest::Debian(m) => return Some(m.get_type()),
                _ => return None,
            },
            None => return None,
        }
    }

    pub fn load_from_file(path: String) -> Option<AbstractManifest> {
        let file_path = path::Path::new(&path);
        if !file_path.is_file() {
            eprintln!("{} is not a file.", path);
            return None;
        }

        let manifest_content = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("could not read manifest file {}: {}.", path, e);
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
        } else if let Some(js_manifest) = crate::manifests::javascript::JavascriptPackageManifest::parse(&manifest_content) {
            native_manifest = Some(NativeManifest::Javascript(js_manifest));
        } else {
            return None;
        }

        let mut manifest_format = ManifestFormat::TEXT;
        if path.ends_with(".json") {
            manifest_format = ManifestFormat::JSON;
        } else if path.ends_with(".yaml") || path.ends_with(".yml") {
            manifest_format = ManifestFormat::YAML;
        }

        let manifest = AbstractManifest {
            path: path,
            format: manifest_format,
            native_manifest: native_manifest,
        };
        log::info!("Successfully parsed manifest of type `{}`.", manifest.get_type().unwrap_or("unknown"));
        log::debug!("Parsed manifest. Resulting manifest is {:#?}", &manifest);
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

    pub fn run_command(&self, command: &str) -> Result<String, String> {
        let output = match &self.native_manifest {
            Some(n) => match n {
                NativeManifest::Flatpak(m) => crate::manifests::flatpak::run_command(self, command),
                _ => return Err("Running a command is not supported for this manifest format.".to_string()),
            },
            None => return Err("No manifest to run the command with!".to_string()),
        };
        match output {
            Ok(o) => Ok(o),
            Err(e) => Err(e.to_string()),
        }
    }
}

