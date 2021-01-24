use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(default)]
pub struct CargoManifest {
    package: CargoPackage,
    dependencies: CargoDependencies,
}
impl CargoManifest {
    pub fn get_type(&self) -> &str {
        return "cargo";
    }

    pub fn parse(manifest_content: &String) -> Option<CargoManifest> {
        let cargo_manifest: CargoManifest = match toml::from_str(&manifest_content) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Failed to parse the Cargo manifest: {}.", e);
                return None;
            }
        };

        if cargo_manifest.package.name.is_empty() {
            eprintln!("Required package name is missing from cargo manifest.");
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
