use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(default)]
pub struct CargoManifest {
    pub package: CargoPackage,
    pub dependencies: BTreeMap<String, CargoDependency>,
}
impl CargoManifest {
    pub fn get_type(&self) -> &str {
        return "cargo";
    }

    pub fn file_path_matches(path: &str) -> bool {
        if path.to_lowercase().ends_with("cargo.toml") {
            return true;
        }
        return false;
    }

    pub fn parse(manifest_content: &String) -> Option<CargoManifest> {
        let cargo_manifest: CargoManifest = match toml::from_str(&manifest_content) {
            Ok(m) => m,
            Err(e) => {
                log::debug!("Failed to parse the Cargo manifest: {}.", e);
                return None;
            }
        };

        if cargo_manifest.package.name.is_empty() {
            log::debug!("Required package name is missing from cargo manifest.");
            return None;
        }

        Some(cargo_manifest)
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(default)]
pub struct CargoPackage {
    pub name: String,
    // TODO use a semver.
    pub version: String,
    pub authors: Vec<String>,
    // The rust edition.
    pub edition: String,
    pub description: String,
    pub license: String,
    pub repository: String,
    // The path of the readme file.
    pub readme: String,
    // The is a limit of 5 keywords per package.
    pub keywords: Vec<String>,
    // The list of files to include when publishing the package.
    pub include: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(default)]
pub struct CargoDependencies {}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CargoDependency {
    Version(String),
    VersionObject(CargoVersionObject),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CargoVersionObject {
    pub version: String,
    pub features: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_file_path_matches() {
        assert!(CargoManifest::file_path_matches("Cargo.toml"));
        assert!(CargoManifest::file_path_matches("./Cargo.toml"));
        assert!(CargoManifest::file_path_matches("./path/to/the/Cargo.toml"));
        assert!(!CargoManifest::file_path_matches("com.example.appName.yaml"));
        assert!(!CargoManifest::file_path_matches(""));
        assert!(!CargoManifest::file_path_matches("/////////////"));
    }
}
