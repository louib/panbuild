use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct JavascriptPackage {
    pub name: String,

    // FIXME could be translated to a semver.
    pub version: String,
    pub description: String,
    pub typings: String,

    pub repository: String,

    pub files: Vec<String>,
}
