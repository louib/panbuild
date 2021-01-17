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

    pub repository: String,

    pub files: Vec<String>,

    pub license: String,
    pub author: JavascriptPackageAuthor,

    pub dependencies: BTreeMap<String, String>,
    pub dev_dependencies: BTreeMap<String, String>,

    pub scripts: BTreeMap<String, String>,
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
        if js_package_manifest.version.is_empty() {
            log::debug!("Required top-level field version is missing from the Javascript package manifest.");
            return None;
        }
        if js_package_manifest.repository.is_empty() {
            log::debug!("Required top-level field repository is missing from the Javascript package manifest.");
            return None;
        }
        if js_package_manifest.scripts.len() == 0 {
            log::debug!("The scripts section is missing from the Javascript package manifest.");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parse() {
        match JavascriptPackageManifest::parse(
            &r###"
            {
              "name": "user/package",
              "version": "3.1.110",
              "repository": "https://github.com/user/package",
              "engines": {
                "node": ">=12.9.0",
                "npm": ">=6.10.2"
              },
              "files": [
                "dist/src"
              ],
              "description": "The Package from the User",
              "main": "dist/src/index.js",
              "typings": "dist/src/index.d.ts",
              "scripts": {
                "test": "npm run compile && NODE_ENV=test nyc mocha --config test/mocharc.js 'dist/test/**/*.js'"
              },
              "pre-commit": [
                "lint",
                "test"
              ],
              "author": {
                "name": "User",
                "email": "hello@example.io"
              },
              "license": "LicenseRef-LICENSE",
              "devDependencies": {
                "@types/chai": "4.x",
                "@types/mocha": "8.x",
                "@types/sinon": "7.0.1",
                "chai": "4.x",
                "mocha": "8.x",
                "pre-commit": "1.x",
                "sinon": "7.x"
              },
              "dependencies": {
                "@types/bluebird": "3.x",
                "@types/lodash": "4.x",
                "@types/node": "12.x",
                "bluebird": "3.x",
                "lodash": "4.x"
              }
            }

        "###
            .to_string(),
        ) {
            None => panic!("Error while parsing the flatpak manifest."),
            Some(manifest) => {
                assert_eq!(manifest.name, "user/package");
                assert_eq!(manifest.repository, "https://github.com/user/package");
                assert_eq!(manifest.version, "3.1.110");
            }
        }
    }
}
