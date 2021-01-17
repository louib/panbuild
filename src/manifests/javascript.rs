use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct JavascriptPackageManifest {
    pub name: String,

    // FIXME could be translated to a semver.
    pub version: String,
    pub description: String,
    pub typings: String,

    pub repository: String,

    pub files: Vec<String>,

    pub license: String,
    pub author: JavascriptPackageAuthor,

    pub dependencies: BTreeMap<String, String>,
    pub dev_dependencies: BTreeMap<String, String>,
}
impl JavascriptPackageManifest {
    pub fn parse(manifest_content: &String) -> Option<JavascriptPackageManifest> {
        let js_package_manifest: JavascriptPackageManifest = match serde_json::from_str(&manifest_content) {
            Ok(m) => m,
            Err(e) => {
                log::debug!("Failed to parse the Javascript package manifest: {}.", e);
                return None;
            }
        };

        // TODO I think there's other fields to validate here.
        if js_package_manifest.name.is_empty() {
            log::debug!("Required top-level field name is missing from the Javascript package manifest.");
            return None;
        }

        Some(js_package_manifest)
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct JavascriptPackageAuthor {
    pub name: String,
    pub email: String,
}
